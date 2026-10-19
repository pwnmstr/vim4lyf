use anyhow::{Context, Result};
use atspi::Role;
use log::info;
use zbus::Connection;

#[derive(Debug, Clone)]
pub struct InteractableElement {
    pub name: String,
    pub role: Role,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub path: String,
    pub bus_name: String,
}

pub struct Scanner {
    conn: Connection,
}

impl Scanner {
    pub async fn new() -> Result<Self> {
        let conn = Connection::session()
            .await
            .context("Failed to connect to session bus")?;
        Ok(Self { conn })
    }

    pub async fn scan_active_window(&self) -> Result<Vec<InteractableElement>> {
        let mut elements = Vec::new();
        info!("Connected to D-Bus. Scanning for interactable elements via AT-SPI...");

        elements.push(InteractableElement {
            name: "Applications Menu".into(),
            role: Role::Menu,
            x: 50,
            y: 50,
            width: 100,
            height: 30,
            path: "/mock/appmenu".into(),
            bus_name: "org.a11y.mock".into(),
        });

        elements.push(InteractableElement {
            name: "Firefox Browser".into(),
            role: Role::Application,
            x: 400,
            y: 200,
            width: 800,
            height: 600,
            path: "/mock/firefox".into(),
            bus_name: "org.a11y.mock".into(),
        });

        elements.push(InteractableElement {
            name: "Close Button".into(),
            role: Role::Icon,
            x: 1150,
            y: 210,
            width: 30,
            height: 30,
            path: "/mock/closebtn".into(),
            bus_name: "org.a11y.mock".into(),
        });

        Ok(elements)
    }
}
