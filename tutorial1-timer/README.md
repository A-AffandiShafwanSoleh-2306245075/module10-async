# Tutorial 1: Timer

## Experiment 1.1: Original timer from the book

Program ini mengimplementasikan async executor sederhana dari buku 
Asynchronous Programming in Rust. Program mencetak "howdy!" lalu 
menunggu 2 detik menggunakan TimerFuture sebelum mencetak "done!".

## Experiment 1.2: Understanding how it works

![Experiment 1.2](assets/screenshot-1.2.png)

Setelah menambahkan `println!("hey hey!")` setelah `spawner.spawn()`,
output yang muncul adalah:
```
Affandi's Komputer: howdy!
Affandi's Komputer: hey hey!
Affandi's Komputer: done!
```

Hal ini terjadi karena `spawner.spawn()` hanya mendaftarkan task ke dalam queue executor tanpa langsung menjalankannya. Task baru benar-benar dijalankan ketika `executor.run()` dipanggil di akhir fungsi main. Oleh karena itu, `println!("hey hey!")` yang berada di luar async block dieksekusi terlebih dahulu setelah spawn selesai mendaftarkan task, tetapi sebelum executor mulai memproses task tersebut. Saat executor mulai berjalan, ia memproses task secara berurutan yaitu mencetak "howdy!", menunggu timer 2 detik, lalu mencetak "done!". Ini membuktikan bahwa spawner dan executor bekerja secara terpisah dimana spawner hanya mendaftarkan task sedangkan executor yang benar-benar menjalankannya.
