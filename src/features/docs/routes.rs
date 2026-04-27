use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

use crate::state::SharedState;

pub fn router() -> Router<SharedState> {
    Router::<SharedState>::new()
        .route("/", get(docs_home))
        .route("/openapi.json", get(openapi_spec))
}

async fn docs_home() -> impl IntoResponse {
    Html(DOCS_HTML)
}

async fn openapi_spec() -> Json<Value> {
    Json(json!({
        "openapi": "3.1.0",
        "info": {
            "title": "trapi",
            "version": "0.1.0",
            "description": "Track fitness progress through a simple authenticated API."
        },
        "servers": [
            { "url": "https://trapi-pink.vercel.app", "description": "Production" },
            { "url": "http://127.0.0.1:3000", "description": "Local development" }
        ],
        "tags": [
            { "name": "Docs", "description": "Documentation and API discovery" },
            { "name": "Health", "description": "Service and database health" },
            { "name": "Auth", "description": "User registration, login, and profile" },
            { "name": "Workouts", "description": "Authenticated workout creation" }
        ],
        "components": {
            "securitySchemes": {
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT"
                }
            },
            "schemas": {
                "HealthResponse": {
                    "type": "object",
                    "required": ["status", "database"],
                    "properties": {
                        "status": { "type": "string", "example": "ok" },
                        "database": { "type": "string", "example": "ok" }
                    }
                },
                "RegisterRequest": {
                    "type": "object",
                    "required": ["email", "username", "password"],
                    "properties": {
                        "email": { "type": "string", "format": "email", "example": "runner@example.com" },
                        "username": { "type": "string", "example": "runner01" },
                        "password": { "type": "string", "format": "password", "example": "Passw0rd!123" }
                    }
                },
                "User": {
                    "type": "object",
                    "required": ["id", "email", "username"],
                    "properties": {
                        "id": { "type": "string", "format": "uuid" },
                        "email": { "type": "string", "format": "email" },
                        "username": { "type": "string" }
                    }
                },
                "LoginRequest": {
                    "type": "object",
                    "required": ["email", "password"],
                    "properties": {
                        "email": { "type": "string", "format": "email", "example": "runner@example.com" },
                        "password": { "type": "string", "format": "password", "example": "Passw0rd!123" }
                    }
                },
                "LoginResponse": {
                    "type": "object",
                    "required": ["token"],
                    "properties": {
                        "token": { "type": "string", "example": "eyJhbGciOiJIUzI1NiJ9..." }
                    }
                },
                "MeResponse": {
                    "type": "object",
                    "required": ["id", "email", "username", "total_workout_count"],
                    "properties": {
                        "id": { "type": "string", "format": "uuid" },
                        "email": { "type": "string", "format": "email" },
                        "username": { "type": "string" },
                        "total_workout_count": { "type": "integer", "format": "int64", "example": 1 }
                    }
                },
                "CreateWorkoutRequest": {
                    "type": "object",
                    "required": ["workout_type"],
                    "properties": {
                        "workout_type": {
                            "type": "string",
                            "enum": ["run", "bike", "swim", "hike", "lift", "row"],
                            "example": "run"
                        },
                        "distance_km": { "type": "number", "format": "double", "example": 5.25 },
                        "duration_seconds": { "type": "integer", "format": "int32", "example": 1620 },
                        "notes": { "type": "string", "example": "Evening recovery run" }
                    }
                },
                "Workout": {
                    "type": "object",
                    "required": ["id", "user_id", "workout_type"],
                    "properties": {
                        "id": { "type": "string", "format": "uuid" },
                        "user_id": { "type": "string", "format": "uuid" },
                        "workout_type": { "type": "string", "example": "run" },
                        "distance_km": { "type": ["number", "null"], "format": "double", "example": 5.25 },
                        "duration_seconds": { "type": ["integer", "null"], "format": "int32", "example": 1620 },
                        "notes": { "type": ["string", "null"], "example": "Evening recovery run" }
                    }
                }
            }
        },
        "paths": {
            "/": {
                "get": {
                    "tags": ["Docs"],
                    "summary": "Open interactive API docs",
                    "responses": {
                        "200": {
                            "description": "Swagger-style documentation page",
                            "content": {
                                "text/html": {}
                            }
                        }
                    }
                }
            },
            "/openapi.json": {
                "get": {
                    "tags": ["Docs"],
                    "summary": "OpenAPI specification",
                    "responses": {
                        "200": {
                            "description": "Machine-readable API spec",
                            "content": {
                                "application/json": {}
                            }
                        }
                    }
                }
            },
            "/health": {
                "get": {
                    "tags": ["Health"],
                    "summary": "Check service and database health",
                    "responses": {
                        "200": {
                            "description": "Service and database are healthy",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/HealthResponse" }
                                }
                            }
                        },
                        "503": {
                            "description": "Database health check failed"
                        }
                    }
                }
            },
            "/auth/register": {
                "post": {
                    "tags": ["Auth"],
                    "summary": "Register a new user",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/RegisterRequest" }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "User created",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/User" }
                                }
                            }
                        },
                        "400": { "description": "Invalid request body" },
                        "409": { "description": "Email or username already exists" }
                    }
                }
            },
            "/auth/login": {
                "post": {
                    "tags": ["Auth"],
                    "summary": "Authenticate and receive a JWT",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/LoginRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Login successful",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/LoginResponse" }
                                }
                            }
                        },
                        "400": { "description": "Invalid request body" },
                        "401": { "description": "Invalid credentials" }
                    }
                }
            },
            "/auth/me": {
                "get": {
                    "tags": ["Auth"],
                    "summary": "Get the authenticated user profile",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Authenticated profile",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/MeResponse" }
                                }
                            }
                        },
                        "401": { "description": "Missing or invalid bearer token" },
                        "404": { "description": "User not found" }
                    }
                }
            },
            "/workouts": {
                "post": {
                    "tags": ["Workouts"],
                    "summary": "Create a workout for the authenticated user",
                    "security": [{ "bearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/CreateWorkoutRequest" }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "Workout created",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/Workout" }
                                }
                            }
                        },
                        "400": { "description": "Invalid workout request" },
                        "401": { "description": "Missing or invalid bearer token" }
                    }
                }
            }
        }
    }))
}

const DOCS_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>trapi docs</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
    <style>
      :root {
        color-scheme: light;
      }

      body {
        margin: 0;
        font-family: "IBM Plex Sans", "Helvetica Neue", sans-serif;
        background:
          radial-gradient(circle at top left, #d7efe3 0, transparent 35%),
          linear-gradient(180deg, #f3f7f2 0%, #ffffff 60%);
        color: #17201c;
      }

      .hero {
        padding: 40px 24px 20px;
        max-width: 1080px;
        margin: 0 auto;
      }

      .eyebrow {
        display: inline-block;
        font-size: 12px;
        letter-spacing: 0.14em;
        text-transform: uppercase;
        color: #2d6a4f;
        margin-bottom: 12px;
      }

      h1 {
        font-size: clamp(36px, 6vw, 64px);
        line-height: 0.95;
        margin: 0 0 14px;
      }

      .lede {
        max-width: 760px;
        font-size: 18px;
        line-height: 1.6;
        color: #385046;
        margin: 0;
      }

      .quick-links {
        display: flex;
        gap: 12px;
        flex-wrap: wrap;
        margin-top: 22px;
      }

      .quick-links a {
        text-decoration: none;
        color: #173126;
        background: rgba(255, 255, 255, 0.85);
        border: 1px solid rgba(45, 106, 79, 0.18);
        border-radius: 999px;
        padding: 10px 14px;
        font-size: 14px;
      }

      #swagger-ui {
        max-width: 1200px;
        margin: 0 auto 48px;
        padding: 0 12px 32px;
      }

      .swagger-ui .topbar {
        display: none;
      }

      .swagger-ui .info {
        margin: 0 0 24px;
      }
    </style>
  </head>
  <body>
    <section class="hero">
      <div class="eyebrow">trapi</div>
      <h1>Fitness API docs</h1>
      <p class="lede">
        Register users, issue JWTs, inspect the current profile, and create workouts.
        The interactive reference below is driven by the live OpenAPI spec served from this app.
      </p>
      <div class="quick-links">
        <a href="/openapi.json">OpenAPI JSON</a>
        <a href="/health">Health Check</a>
        <a href="https://github.com/kashsuks/trapi">GitHub Repo</a>
      </div>
    </section>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
    <script>
      window.ui = SwaggerUIBundle({
        url: "/openapi.json",
        dom_id: "#swagger-ui",
        deepLinking: true,
        displayRequestDuration: true,
        persistAuthorization: true,
        tryItOutEnabled: true,
      });
    </script>
  </body>
</html>
"##;
