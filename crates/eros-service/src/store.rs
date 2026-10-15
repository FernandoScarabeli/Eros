use eros_core::{
    ChatActivityEvent, ChatStatus, Conversation, Layout, Project, SessionKind, SessionState,
    SidebarGroup, SidebarOrders, Snapshot, Workspace, PROTOCOL,
};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

pub struct Store {
    connection: Mutex<Connection>,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self, String> {
        let connection = Connection::open(path).map_err(|error| error.to_string())?;
        connection
            .execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;
                 PRAGMA foreign_keys = ON;
                 CREATE TABLE IF NOT EXISTS projects (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    path TEXT NOT NULL UNIQUE
                 );
                 CREATE TABLE IF NOT EXISTS conversations (
                    id TEXT PRIMARY KEY,
                    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
                    name TEXT NOT NULL,
                    cwd TEXT NOT NULL,
                    kind TEXT NOT NULL,
                    state TEXT NOT NULL,
                    provider_thread_id TEXT,
                    pinned INTEGER NOT NULL DEFAULT 0
                 );
                 CREATE TABLE IF NOT EXISTS workspaces (
                    scope TEXT PRIMARY KEY,
                    document TEXT NOT NULL
                 );
                 CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                 );",
            )
            .map_err(|error| error.to_string())?;
        let has_pinned = {
            let mut statement = connection
                .prepare("PRAGMA table_info(conversations)")
                .map_err(|error| error.to_string())?;
            let columns = statement
                .query_map([], |row| row.get::<_, String>(1))
                .map_err(|error| error.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.to_string())?;
            columns.iter().any(|column| column == "pinned")
        };
        if !has_pinned {
            connection
                .execute(
                    "ALTER TABLE conversations ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0",
                    [],
                )
                .map_err(|error| error.to_string())?;
        }
        let has_chat_status = connection
            .prepare("PRAGMA table_info(conversations)")
            .map_err(|error| error.to_string())?
            .query_map([], |row| row.get::<_, String>(1))
            .map_err(|error| error.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?
            .iter()
            .any(|column| column == "chat_status");
        if !has_chat_status {
            connection.execute(
                "ALTER TABLE conversations ADD COLUMN chat_status TEXT NOT NULL DEFAULT 'normal'",
                [],
            ).map_err(|error| error.to_string())?;
        }
        let has_revision = connection
            .prepare("PRAGMA table_info(conversations)")
            .map_err(|error| error.to_string())?
            .query_map([], |row| row.get::<_, String>(1))
            .map_err(|error| error.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?
            .iter()
            .any(|column| column == "activity_revision");
        if !has_revision {
            connection.execute(
                "ALTER TABLE conversations ADD COLUMN activity_revision INTEGER NOT NULL DEFAULT 0", [],
            ).map_err(|error| error.to_string())?;
        }
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn snapshot(&self) -> Result<Snapshot, String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        connection.execute(
            "DELETE FROM conversations WHERE kind = 'shell' AND state NOT IN ('running', 'starting')",
            [],
        ).map_err(|error| error.to_string())?;
        let mut projects_statement = connection
            .prepare("SELECT id, name, path FROM projects ORDER BY name COLLATE NOCASE")
            .map_err(|error| error.to_string())?;
        let projects = projects_statement
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                })
            })
            .map_err(|error| error.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;

        let mut conversations_statement = connection
            .prepare(
                "SELECT id, project_id, name, cwd, kind, state, provider_thread_id, pinned, chat_status, activity_revision
                 FROM conversations ORDER BY rowid DESC",
            )
            .map_err(|error| error.to_string())?;
        let conversations = conversations_statement
            .query_map([], |row| {
                let kind: String = row.get(4)?;
                let state: String = row.get(5)?;
                Ok(Conversation {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    name: row.get(2)?,
                    cwd: row.get(3)?,
                    kind: parse_kind(&kind).map_err(to_sql_conversion_error)?,
                    state: parse_state(&state).map_err(to_sql_conversion_error)?,
                    provider_thread_id: row.get(6)?,
                    pinned: row.get(7)?,
                    activity_revision: row.get(9)?,
                    chat_status: {
                        let value: String = row.get(8)?;
                        serde_json::from_value(serde_json::Value::String(value))
                            .map_err(|error| to_sql_conversion_error(error.to_string()))?
                    },
                })
            })
            .map_err(|error| error.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;

        let mut workspaces_statement = connection
            .prepare("SELECT document FROM workspaces ORDER BY scope")
            .map_err(|error| error.to_string())?;
        let mut workspaces = workspaces_statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|error| error.to_string())?
            .map(|document| {
                let document = document.map_err(|error| error.to_string())?;
                serde_json::from_str::<Workspace>(&document).map_err(|error| error.to_string())
            })
            .collect::<Result<Vec<_>, String>>()?;

        for workspace in &mut workspaces {
            workspace.panels.retain(|panel| {
                conversations
                    .iter()
                    .any(|item| item.id == panel.conversation_id)
            });
            workspace.tree = workspace
                .tree
                .take()
                .and_then(|tree| prune_layout(tree, &workspace.panels));
            if !workspace
                .panels
                .iter()
                .any(|panel| Some(&panel.id) == workspace.focused_panel_id.as_ref())
            {
                workspace.focused_panel_id = workspace.panels.last().map(|panel| panel.id.clone());
            }
        }

        migrate_agent_workspaces(&connection, &mut workspaces, &conversations)?;

        let active_scope = connection
            .query_row(
                "SELECT value FROM settings WHERE key = 'active_scope'",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| error.to_string())?
            .unwrap_or_else(|| "chats".into());

        let sidebar_orders = SidebarOrders {
            projects: read_sidebar_order(&connection, "sidebar_order_projects"),
            chats: read_sidebar_order(&connection, "sidebar_order_chats"),
        };

        let chat_default_folder = connection
            .query_row(
                "SELECT value FROM settings WHERE key = 'chat_default_folder'",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| error.to_string())?
            .unwrap_or_default();
        let chat_default_folder_unavailable = !chat_default_folder.is_empty()
            && std::fs::canonicalize(&chat_default_folder).map_or(true, |path| !path.is_dir());

        Ok(Snapshot {
            protocol: PROTOCOL,
            projects,
            conversations,
            workspaces,
            sidebar_orders,
            authenticated_providers: Vec::new(),
            active_scope,
            chat_default_folder,
            chat_default_folder_unavailable,
        })
    }

    pub fn resolve_chat_default_folder(&self) -> Result<String, String> {
        let configured = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?
            .query_row(
                "SELECT value FROM settings WHERE key = 'chat_default_folder'",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| error.to_string())?
            .unwrap_or_default();
        if !configured.is_empty() {
            if let Ok(path) = std::fs::canonicalize(&configured) {
                if path.is_dir() {
                    return Ok(path.to_string_lossy().into_owned());
                }
            }
        }
        let home = crate::paths::home_dir()
            .map_err(|error| format!("A pasta pessoal não está disponível: {error}"))?;
        Ok(home.to_string_lossy().into_owned())
    }

    pub fn set_chat_default_folder(&self, path: &str) -> Result<(), String> {
        let path = if path.is_empty() {
            String::new()
        } else {
            let folder = std::fs::canonicalize(path)
                .map_err(|error| format!("A pasta escolhida não está acessível: {error}"))?;
            if !folder.is_dir() {
                return Err("Escolha uma pasta válida.".into());
            }
            folder.to_string_lossy().into_owned()
        };
        self.connection.lock().map_err(|_| "O banco local ficou indisponível.")?
            .execute("INSERT INTO settings (key, value) VALUES ('chat_default_folder', ?1) ON CONFLICT(key) DO UPDATE SET value = excluded.value", [&path])
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub fn add_project(&self, path: &str) -> Result<Project, String> {
        let canonical = std::fs::canonicalize(path)
            .map_err(|_| "Não foi possível abrir essa pasta. Verifique se ela ainda existe.")?;
        if !canonical.is_dir() {
            return Err("Escolha uma pasta de projeto.".into());
        }
        let path = canonical.to_string_lossy().into_owned();
        let name = canonical
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .unwrap_or("Projeto")
            .to_owned();
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        if let Some(project) = connection
            .query_row(
                "SELECT id, name, path FROM projects WHERE path = ?1",
                [&path],
                |row| {
                    Ok(Project {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        path: row.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(|error| error.to_string())?
        {
            return Ok(project);
        }

        let project = Project {
            id: Uuid::now_v7().to_string(),
            name,
            path,
        };
        connection
            .execute(
                "INSERT INTO projects (id, name, path) VALUES (?1, ?2, ?3)",
                params![project.id, project.name, project.path],
            )
            .map_err(|error| error.to_string())?;
        Ok(project)
    }

    pub fn rename_project(&self, id: &str, name: &str) -> Result<Project, String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("Digite um nome para o projeto.".into());
        }
        if name.chars().count() > 80 {
            return Err("O nome do projeto deve ter até 80 caracteres.".into());
        }

        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        let changed = connection
            .execute(
                "UPDATE projects SET name = ?1 WHERE id = ?2",
                params![name, id],
            )
            .map_err(|error| error.to_string())?;
        if changed == 0 {
            return Err("Este projeto não existe mais.".into());
        }

        connection
            .query_row(
                "SELECT id, name, path FROM projects WHERE id = ?1",
                [id],
                |row| {
                    Ok(Project {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        path: row.get(2)?,
                    })
                },
            )
            .map_err(|error| error.to_string())
    }

    pub fn set_sidebar_order(
        &self,
        group: SidebarGroup,
        ids: &[String],
    ) -> Result<SidebarOrders, String> {
        let (query, key, label) = match group {
            SidebarGroup::Projects => (
                "SELECT id FROM projects ORDER BY name COLLATE NOCASE",
                "sidebar_order_projects",
                "projetos",
            ),
            SidebarGroup::Chats => (
                "SELECT id FROM conversations WHERE project_id IS NULL AND kind != 'shell' ORDER BY rowid DESC",
                "sidebar_order_chats",
                "chats",
            ),
        };
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        let mut statement = connection
            .prepare(query)
            .map_err(|error| error.to_string())?;
        let current = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|error| error.to_string())?
            .collect::<Result<std::collections::HashSet<_>, _>>()
            .map_err(|error| error.to_string())?;
        let requested = ids
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>();
        if current.len() != ids.len() || current != requested {
            return Err(format!(
                "A ordem de {label} está desatualizada. Atualize a lista e tente novamente."
            ));
        }

        let serialized = serde_json::to_string(ids).map_err(|error| error.to_string())?;
        connection
            .execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![key, serialized],
            )
            .map_err(|error| error.to_string())?;
        Ok(SidebarOrders {
            projects: read_sidebar_order(&connection, "sidebar_order_projects"),
            chats: read_sidebar_order(&connection, "sidebar_order_chats"),
        })
    }

    pub fn create_conversation(&self, conversation: &Conversation) -> Result<(), String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        if let Some(project_id) = &conversation.project_id {
            let project_path = connection
                .query_row(
                    "SELECT path FROM projects WHERE id = ?1",
                    [project_id],
                    |row| row.get::<_, String>(0),
                )
                .optional()
                .map_err(|error| error.to_string())?
                .ok_or_else(|| "O projeto selecionado não existe mais.".to_string())?;
            if !Path::new(&conversation.cwd).starts_with(Path::new(&project_path)) {
                return Err(
                    "A pasta de trabalho precisa estar dentro do projeto selecionado.".into(),
                );
            }
        }
        connection
            .execute(
                "INSERT INTO conversations
                    (id, project_id, name, cwd, kind, state, provider_thread_id, pinned, chat_status, activity_revision)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    conversation.id,
                    conversation.project_id,
                    conversation.name,
                    conversation.cwd,
                    encode_kind(&conversation.kind),
                    encode_state(&conversation.state),
                    conversation.provider_thread_id,
                    conversation.pinned,
                    serde_json::to_value(&conversation.chat_status).unwrap().as_str().unwrap(),
                    conversation.activity_revision,
                ],
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub fn save_workspace(&self, workspace: &Workspace) -> Result<(), String> {
        let document = serde_json::to_string(workspace).map_err(|error| error.to_string())?;
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        connection
            .execute(
                "INSERT INTO workspaces (scope, document) VALUES (?1, ?2)
                 ON CONFLICT(scope) DO UPDATE SET document = excluded.document",
                params![workspace.scope, document],
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub fn set_active_scope(&self, scope: &str) -> Result<(), String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        if let Some(conversation_id) = scope.strip_prefix("conversation:") {
            let exists = connection
                .query_row(
                    "SELECT 1 FROM conversations WHERE id = ?1 AND kind IN ('codex', 'claude')",
                    [conversation_id],
                    |_| Ok(()),
                )
                .optional()
                .map_err(|error| error.to_string())?
                .is_some();
            if !exists {
                return Err("Este chat não existe mais.".into());
            }
        } else if scope != "chats" {
            let exists = connection
                .query_row("SELECT 1 FROM projects WHERE id = ?1", [scope], |_| Ok(()))
                .optional()
                .map_err(|error| error.to_string())?
                .is_some();
            if !exists {
                return Err("O projeto selecionado não existe mais.".into());
            }
        }
        connection
            .execute(
                "INSERT INTO settings (key, value) VALUES ('active_scope', ?1)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                [scope],
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub fn set_chat_activity(
        &self,
        id: &str,
        status: &ChatStatus,
        provider_session: Option<&str>,
    ) -> Result<ChatActivityEvent, String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        let status = serde_json::to_value(status).map_err(|error| error.to_string())?;
        connection.execute(
            "UPDATE conversations SET chat_status = ?2, activity_revision = activity_revision + 1,
             provider_thread_id = COALESCE(?3, provider_thread_id) WHERE id = ?1 AND kind != 'shell'",
            params![id, status.as_str(), provider_session],
        ).map_err(|error| error.to_string())?;
        read_activity(&connection, id)
    }

    pub fn mark_conversation_read(
        &self,
        id: &str,
        revision: u64,
    ) -> Result<ChatActivityEvent, String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        connection.execute(
            "UPDATE conversations SET chat_status = 'normal', activity_revision = activity_revision + 1
             WHERE id = ?1 AND chat_status = 'completed_unread' AND activity_revision = ?2",
            params![id, revision],
        ).map_err(|error| error.to_string())?;
        read_activity(&connection, id)
    }

    pub fn activity_snapshot(&self) -> Result<Vec<ChatActivityEvent>, String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        let mut statement = connection.prepare(
            "SELECT id, chat_status, activity_revision FROM conversations WHERE kind != 'shell'",
        ).map_err(|error| error.to_string())?;
        let rows = statement
            .query_map([], activity_row)
            .map_err(|error| error.to_string())?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())
    }

    pub fn set_conversation_state(
        &self,
        conversation_id: &str,
        state: &SessionState,
    ) -> Result<(), String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        connection
            .execute(
                "UPDATE conversations SET state = ?2 WHERE id = ?1",
                params![conversation_id, encode_state(state)],
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub fn set_conversation_pinned(&self, id: &str, pinned: bool) -> Result<(), String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        let changed = connection
            .execute(
                "UPDATE conversations SET pinned = ?2
                 WHERE id = ?1 AND project_id IS NULL AND kind IN ('codex', 'claude')",
                params![id, pinned],
            )
            .map_err(|error| error.to_string())?;
        if changed == 0 {
            return Err("Este chat não existe mais.".into());
        }
        Ok(())
    }

    pub fn ensure_standalone_chat(&self, id: &str) -> Result<(), String> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        let exists = connection
            .query_row(
                "SELECT 1 FROM conversations
                 WHERE id = ?1 AND project_id IS NULL AND kind IN ('codex', 'claude')",
                [id],
                |_| Ok(()),
            )
            .optional()
            .map_err(|error| error.to_string())?
            .is_some();
        if !exists {
            return Err("Este chat não existe mais.".into());
        }
        Ok(())
    }

    pub fn delete_standalone_conversation(&self, id: &str) -> Result<(), String> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| "O banco local ficou indisponível.")?;
        let transaction = connection
            .transaction()
            .map_err(|error| error.to_string())?;
        let scope = format!("conversation:{id}");
        let changed = transaction
            .execute(
                "DELETE FROM conversations
                 WHERE id = ?1 AND project_id IS NULL AND kind IN ('codex', 'claude')",
                [id],
            )
            .map_err(|error| error.to_string())?;
        if changed == 0 {
            return Err("Este chat não existe mais.".into());
        }

        transaction
            .execute("DELETE FROM workspaces WHERE scope = ?1", [&scope])
            .map_err(|error| error.to_string())?;
        let workspace_documents = {
            let mut statement = transaction
                .prepare("SELECT scope, document FROM workspaces")
                .map_err(|error| error.to_string())?;
            let rows = statement
                .query_map([], |row| {
                    Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
                })
                .map_err(|error| error.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.to_string())?;
            rows
        };
        for (workspace_scope, document) in workspace_documents {
            let mut workspace: Workspace =
                serde_json::from_str(&document).map_err(|error| error.to_string())?;
            let panel_count = workspace.panels.len();
            workspace.panels.retain(|panel| panel.conversation_id != id);
            if workspace.panels.len() == panel_count {
                continue;
            }
            workspace.tree = workspace
                .tree
                .take()
                .and_then(|tree| prune_layout(tree, &workspace.panels));
            if !workspace
                .panels
                .iter()
                .any(|panel| Some(&panel.id) == workspace.focused_panel_id.as_ref())
            {
                workspace.focused_panel_id = workspace.panels.last().map(|panel| panel.id.clone());
            }
            let document = serde_json::to_string(&workspace).map_err(|error| error.to_string())?;
            transaction
                .execute(
                    "UPDATE workspaces SET document = ?2 WHERE scope = ?1",
                    params![workspace_scope, document],
                )
                .map_err(|error| error.to_string())?;
        }

        let mut chat_order = read_sidebar_order(&transaction, "sidebar_order_chats");
        chat_order.retain(|chat_id| chat_id != id);
        let chat_order = serde_json::to_string(&chat_order).map_err(|error| error.to_string())?;
        transaction
            .execute(
                "INSERT INTO settings (key, value) VALUES ('sidebar_order_chats', ?1)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                [chat_order],
            )
            .map_err(|error| error.to_string())?;
        transaction
            .execute(
                "UPDATE settings SET value = 'chats' WHERE key = 'active_scope' AND value = ?1",
                [&scope],
            )
            .map_err(|error| error.to_string())?;
        transaction.commit().map_err(|error| error.to_string())?;
        Ok(())
    }
}

fn prune_layout(tree: Layout, panels: &[eros_core::Panel]) -> Option<Layout> {
    match tree {
        Layout::Leaf { panel_id } => panels
            .iter()
            .any(|panel| panel.id == panel_id)
            .then_some(Layout::Leaf { panel_id }),
        Layout::Split {
            axis,
            ratio,
            first,
            second,
        } => match (prune_layout(*first, panels), prune_layout(*second, panels)) {
            (Some(first), Some(second)) => Some(Layout::Split {
                axis,
                ratio,
                first: Box::new(first),
                second: Box::new(second),
            }),
            (first, second) => first.or(second),
        },
    }
}

fn migrate_agent_workspaces(
    connection: &Connection,
    workspaces: &mut Vec<Workspace>,
    conversations: &[Conversation],
) -> Result<(), String> {
    let mut selected_panels = std::collections::HashMap::new();
    for conversation in conversations
        .iter()
        .filter(|conversation| conversation.kind != SessionKind::Shell)
    {
        let target_scope = format!("conversation:{}", conversation.id);
        let panel = workspaces
            .iter()
            .find(|workspace| workspace.scope == target_scope)
            .and_then(|workspace| {
                workspace
                    .panels
                    .iter()
                    .find(|panel| panel.conversation_id == conversation.id)
            })
            .or_else(|| {
                workspaces.iter().find_map(|workspace| {
                    workspace
                        .panels
                        .iter()
                        .find(|panel| panel.conversation_id == conversation.id)
                })
            });
        if let Some(panel) = panel {
            selected_panels.insert(conversation.id.clone(), panel.clone());
        }
    }

    let mut kept_panels = std::collections::HashSet::new();
    let mut changed = false;
    for workspace in workspaces.iter_mut() {
        let old_count = workspace.panels.len();
        let workspace_scope = workspace.scope.clone();
        workspace.panels.retain(|panel| {
            let Some(conversation) = conversations
                .iter()
                .find(|conversation| conversation.id == panel.conversation_id)
            else {
                return false;
            };
            if conversation.kind == SessionKind::Shell {
                return true;
            }
            let expected_scope = format!("conversation:{}", conversation.id);
            let is_selected = selected_panels
                .get(&conversation.id)
                .is_some_and(|selected| selected.id == panel.id);
            let keep = workspace_scope == expected_scope
                && is_selected
                && kept_panels.insert(panel.id.clone());
            if !keep {
                changed = true;
            }
            keep
        });
        if workspace.panels.len() != old_count {
            changed = true;
            workspace.tree = workspace
                .tree
                .take()
                .and_then(|tree| prune_layout(tree, &workspace.panels));
            if !workspace
                .panels
                .iter()
                .any(|panel| Some(&panel.id) == workspace.focused_panel_id.as_ref())
            {
                workspace.focused_panel_id = workspace.panels.last().map(|panel| panel.id.clone());
            }
        }
    }

    for (conversation_id, panel) in selected_panels {
        let scope = format!("conversation:{conversation_id}");
        if !kept_panels.contains(&panel.id) {
            let workspace = Workspace {
                scope: scope.clone(),
                panels: vec![panel.clone()],
                tree: Some(Layout::Leaf {
                    panel_id: panel.id.clone(),
                }),
                focused_panel_id: Some(panel.id),
            };
            if let Some(existing) = workspaces.iter_mut().find(|item| item.scope == scope) {
                *existing = workspace;
            } else {
                workspaces.push(workspace);
            }
            changed = true;
        }
    }

    if changed {
        for workspace in workspaces.iter() {
            let document = serde_json::to_string(workspace).map_err(|error| error.to_string())?;
            connection
                .execute(
                    "INSERT INTO workspaces (scope, document) VALUES (?1, ?2)
                     ON CONFLICT(scope) DO UPDATE SET document = excluded.document",
                    params![workspace.scope, document],
                )
                .map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn read_sidebar_order(connection: &Connection, key: &str) -> Vec<String> {
    connection
        .query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
            row.get::<_, String>(0)
        })
        .optional()
        .ok()
        .flatten()
        .and_then(|value| serde_json::from_str(&value).ok())
        .unwrap_or_default()
}

fn encode_kind(kind: &SessionKind) -> &'static str {
    match kind {
        SessionKind::Shell => "shell",
        SessionKind::Codex => "codex",
        SessionKind::Claude => "claude",
    }
}

fn parse_kind(value: &str) -> Result<SessionKind, String> {
    match value {
        "shell" => Ok(SessionKind::Shell),
        "codex" => Ok(SessionKind::Codex),
        "claude" => Ok(SessionKind::Claude),
        _ => Err("Tipo de conversa desconhecido.".into()),
    }
}

fn encode_state(state: &SessionState) -> &'static str {
    match state {
        SessionState::Starting => "starting",
        SessionState::Running => "running",
        SessionState::Exited => "exited",
        SessionState::Stopped => "stopped",
        SessionState::Unavailable => "unavailable",
    }
}

fn parse_state(value: &str) -> Result<SessionState, String> {
    match value {
        "starting" => Ok(SessionState::Starting),
        "running" => Ok(SessionState::Running),
        "exited" => Ok(SessionState::Exited),
        "stopped" => Ok(SessionState::Stopped),
        "unavailable" => Ok(SessionState::Unavailable),
        _ => Err("Estado de conversa desconhecido.".into()),
    }
}

fn activity_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ChatActivityEvent> {
    let value: String = row.get(1)?;
    Ok(ChatActivityEvent {
        conversation_id: row.get(0)?,
        chat_status: serde_json::from_value(serde_json::Value::String(value))
            .map_err(|error| to_sql_conversion_error(error.to_string()))?,
        activity_revision: row.get(2)?,
    })
}

fn read_activity(connection: &Connection, id: &str) -> Result<ChatActivityEvent, String> {
    connection.query_row(
        "SELECT id, chat_status, activity_revision FROM conversations WHERE id = ?1 AND kind != 'shell'",
        [id], activity_row,
    ).map_err(|_| "Este chat não existe mais.".into())
}

fn to_sql_conversion_error(error: String) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(
        0,
        rusqlite::types::Type::Text,
        Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, error)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retains_live_shells_but_discards_ended_shells_and_their_panels() {
        let store = Store::open(Path::new(":memory:")).unwrap();
        for (id, kind, state) in [
            ("live", SessionKind::Shell, SessionState::Running),
            ("ended", SessionKind::Shell, SessionState::Exited),
            ("chat", SessionKind::Codex, SessionState::Stopped),
        ] {
            store
                .create_conversation(&Conversation {
                    id: id.into(),
                    project_id: None,
                    name: id.into(),
                    cwd: "/tmp".into(),
                    kind,
                    state,
                    provider_thread_id: None,
                    pinned: false,
                    chat_status: ChatStatus::Normal,
                    activity_revision: 0,
                })
                .unwrap();
        }
        store
            .save_workspace(&Workspace {
                scope: "chats".into(),
                panels: vec![eros_core::Panel {
                    id: "panel".into(),
                    conversation_id: "ended".into(),
                }],
                tree: Some(Layout::Leaf {
                    panel_id: "panel".into(),
                }),
                focused_panel_id: Some("panel".into()),
            })
            .unwrap();
        let snapshot = store.snapshot().unwrap();
        assert_eq!(snapshot.conversations.len(), 2);
        assert!(snapshot.conversations.iter().any(|item| item.id == "live"));
        assert!(snapshot.conversations.iter().any(|item| item.id == "chat"));
        assert!(snapshot.workspaces[0].panels.is_empty());
        assert!(snapshot.workspaces[0].tree.is_none());
        assert!(snapshot.workspaces[0].focused_panel_id.is_none());
        store
            .set_sidebar_order(SidebarGroup::Chats, &["chat".into()])
            .unwrap();
        for state in [SessionState::Stopped, SessionState::Unavailable] {
            store
                .create_conversation(&Conversation {
                    id: "temporary".into(),
                    project_id: None,
                    name: "temporary".into(),
                    cwd: "/tmp".into(),
                    kind: SessionKind::Shell,
                    state: SessionState::Running,
                    provider_thread_id: None,
                    pinned: false,
                    chat_status: ChatStatus::Normal,
                    activity_revision: 0,
                })
                .unwrap();
            store.set_conversation_state("live", &state).unwrap();
            store.set_conversation_state("temporary", &state).unwrap();
            assert!(store
                .snapshot()
                .unwrap()
                .conversations
                .iter()
                .all(|item| item.kind != SessionKind::Shell));
        }
    }
}
