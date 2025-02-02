use crate::manager::ManagerExt;
use std::net::IpAddr;
use tauri::AppHandle;

#[tauri::command]
pub async fn start_client(app: AppHandle, server_ip: IpAddr) {
  app.nil().start_client(server_ip).await;
}

#[tauri::command]
pub async fn stop_client(app: AppHandle) {
  app.nil().stop_client().await;
}
