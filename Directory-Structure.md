rustnexus/
├── src/
│   ├── core/
│   │   ├── app.rs
│   │   ├── error.rs
│   ├── config/
│   │   ├── database.rs
│   │   ├── environment.rs
│   │   └── settings.rs
│   ├── http/
│   │   ├── middleware/
│   │   │   ├── auth.rs
│   │   │   └── cors.rs
│   │   ├── request.rs
│   │   └── response.rs
│   ├── controllers/
│   │   ├── base_controller.rs
│   │   └── user_controller.rs
│   ├── models/
│   │   ├── base_model.rs
│   │   └── user_model.rs
│   ├── services/
│   │   ├── base_service.rs
│   │   └── user_service.rs
│   ├── database/
│   │   ├── migrations/
│   │   │   └── ... (migration files)
│   │   ├── connection.rs
│   │   └── query_builder.rs
│   ├── cache/
│   │   ├── redis.rs
│   │   └── memory.rs
│   ├── jobs/
│   │   ├── queue.rs
│   │   └── worker.rs
│   ├── security/
│   │   ├── authentication.rs
│   │   └── authorization.rs
│   ├── utils/
│   │   ├── logger.rs
│   │   └── validator.rs
│   ├── core.rs
│   ├── config.rs
│   ├── http.rs
│   ├── controllers.rs
│   ├── models.rs
│   ├── services.rs
│   ├── database.rs
│   ├── cache.rs
│   ├── jobs.rs
│   ├── security.rs
│   ├── utils.rs
│   └── lib.rs
├── examples/
│   ├── basic_api/
│   ├── full_stack_app/
│   └── websocket_server/
├── tests/
│   ├── integration/
│   └── unit/
├── docs/
│   ├── api/
│   ├── guides/
│   └── tutorials/
├── cli/
│   └── src/
│       └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
