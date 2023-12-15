use std::io;
use std::collections::HashMap;

fn main() {
    // Menampilkan menu
    println!("Pilih menu:");
    println!("1. Tambah transaksi");
    println!("2. Lihat transaksi");
    println!("3. Edit transaksi");
    println!("4. Hapus transaksi");

    // Meminta input pengguna
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut data_keuangan = HashMap::new();

    // Memproses input pengguna
    match input {
        "1" => {
            // Menambahkan transaksi baru
            println!("Tambahkan transaksi baru:");
            println!("masukan tanggal transaksi");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            data_keuangan.insert(task.trim(), description.trim());
        }
        "2" => {
            // Menampilkan daftar transaksi
            println!("Daftar transaksi:");
            for (task, description) in data_keuangan.iter() {
                println!("* {}: {}", task, description);
            }
        }
        "3" => {
            // Mengedit transaksi
            println!("Edit transaksi:");
            println!("Masukkan transaksi yang ingin diedit:");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            match data_keuangan.get_mut(task.trim()) {
                Some(description) => {
                    // Edit deskripsi transaksi
                    println!("Masukkan deskripsi transaksi yang baru:");
                    let mut new_description = String::new();
                    io::stdin().read_line(&mut new_description).unwrap();
                    *description = new_description.trim();
                }
                None => {
                    // Transaksitidak ditemukan
                    println!("Transaksi tidak ditemukan");
                }
            }
        }
        "4" => {
            // Menghapus transaksi
            println!("Hapus transaksi:");
            println!("Masukkan nama transaksi yang ingin dihapus:");
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            data_keuangan.remove(task.trim());
        }
        _ => {
            // Input tidak valid
            println!("Input tidak valid");
        }
    }
}