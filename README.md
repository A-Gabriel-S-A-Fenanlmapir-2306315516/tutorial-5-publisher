# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. **In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?**

    > Menurut pemahaman saya, dalam kasus **BambangShop** saat ini, penggunaan **Model struct** saja sudah cukup. Hal ini dikarenakan semua subscriber memiliki perilaku dan struktur data yang seragam, yaitu hanya perlu menyimpan `url` dan `name` serta melakukan aksi yang sama (menerima notifikasi via HTTP POST). 
    > 
    > Namun, jika ke depannya sistem ini berkembang dan memiliki berbagai jenis subscriber dengan metode pengiriman yang berbeda (misalnya: SMS, Email, atau Push Notification), maka penggunaan **trait** di Rust akan menjadi sangat penting. Trait akan memungkinkan kita mendefinisikan metode `update()` secara polimorfik, sehingga *Subject* (Publisher) tidak perlu mengetahui detail teknis pengiriman pesan dari masing-masing jenis subscriber.

2. **id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?**

    > Penggunaan **DashMap** jauh lebih tepat dan efisien dibandingkan `Vec` untuk kasus ini karena dua alasan utama:
    > * **Keunikan (Uniqueness):** Map secara otomatis menjamin keunikan kunci (key). Jika menggunakan `Vec`, kita harus melakukan pengecekan manual setiap kali ingin menambah data untuk memastikan tidak ada `url` yang ganda, yang mana proses ini rentan terhadap kesalahan (*human error*).
    > * **Efisiensi Performa:** Dengan menjadikan `url` sebagai kunci di DashMap, proses pencarian atau penghapusan data memiliki kompleksitas waktu rata-rata $O(1)$. Jika menggunakan `Vec`, kita harus melakukan iterasi ke seluruh elemen (Linear Search) dengan kompleksitas $O(n)$, yang akan memperlambat performa aplikasi seiring bertambahnya jumlah subscriber.

3. **When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?**

    > Kita tetap membutuhkan **DashMap** (atau mekanisme penguncian seperti `Mutex<HashMap>`) meskipun kita sudah menerapkan konsep **Singleton** melalui `lazy_static`. 
    > 
    > Perlu dipahami bahwa **Singleton** hanyalah pola desain untuk memastikan hanya ada satu instansi objek di memori, namun pola tersebut tidak secara otomatis menjamin keamanan data saat diakses oleh banyak *thread* secara bersamaan (*thread-safety*). Karena framework Rocket bekerja secara *multithreaded*, beberapa *request* bisa mencoba mengakses atau menulis ke database di waktu yang sama. Tanpa `DashMap` yang menyediakan fitur *concurrent access*, akan terjadi *data race*. `DashMap` memberikan penguncian di tingkat entri (*fine-grained locking*) yang memungkinkan akses aman dan cepat tanpa kita harus mengelola penguncian secara manual.
#### Reflection Publisher-2

#### Reflection Publisher-3
