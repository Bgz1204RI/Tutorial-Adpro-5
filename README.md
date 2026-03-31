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

You can download the Postman Collection JSON here: 

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
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
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

### 1. Do we need a trait (interface) for Subscriber?

In the Observer pattern explained in the Head First Design Patterns book, `Subscriber` is usually defined as an interface because different subscribers might react to updates in different ways. However, in the BambangShop case, the subscriber is only used as a data model that stores information like the subscriber’s `url` and `name`. There is no different behavior between subscribers, and they are all treated the same way by the system.

Because of this, using a single `struct Subscriber` is enough for this implementation. Introducing a trait would make the design more complicated without giving any real benefit for this project. If in the future we wanted different types of subscribers with different notification logic, then using a trait could become useful. But for now, a simple model struct is sufficient.

### 2. Is Vec enough or do we need DashMap?

Using a `Vec` to store subscribers would technically work, but it would not be very efficient. Since the `url` in `Subscriber` is supposed to be unique, using a `Vec` would require us to manually check the list every time we add or delete a subscriber to make sure there are no duplicates. This means we would have to iterate through the list frequently, which becomes inefficient as the number of subscribers grows.

Using `DashMap` is better in this case because it behaves like a key-value dictionary. The `url` can be used as the key, so uniqueness is automatically handled by the map structure. It also allows faster lookup, insertion, and deletion compared to scanning through a `Vec`. Therefore, `DashMap` is more appropriate for this repository design.

### 3. Do we still need DashMap if we use Singleton?

The Singleton pattern ensures that there is only one instance of the subscribers storage in the program. In this implementation, we already apply this idea using a static variable (`SUBSCRIBERS`) so that all parts of the program access the same data.

However, Singleton alone does not solve the problem of thread safety. Since this application may handle multiple requests at the same time, different threads could try to read or modify the subscriber list simultaneously. `DashMap` helps solve this by providing a thread-safe hashmap that can be safely accessed from multiple threads.

Because of that, Singleton and `DashMap` serve different purposes. Singleton ensures there is only one shared data source, while `DashMap` ensures that concurrent access to that data is safe. Therefore, we still need `DashMap` even though the storage behaves like a Singleton.
#### Reflection Publisher-2

#### Reflection Publisher-3
