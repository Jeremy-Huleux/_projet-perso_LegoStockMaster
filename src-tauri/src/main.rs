// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/*!
    © 2025 Jérémy Huleux (Zaelos)
    Logiciel propriétaire – Tous droits réservés
*/
fn main() {
    legostockmaster_lib::run()
}