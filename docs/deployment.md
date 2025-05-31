# Deployment

## Folder Structure

```
projects/
├── container-yard/
│   └── compose.yml
├── project-1/
│   ├── compose.yml
│   └── .env
└── project-2/
    └── compose.yml
```

All folders in the `projects` folder will be seen as a Project and will be displayed in the Overview.

## `compose.yml`

```yml
services:
  frontend:
    image: ghcr.io/tobinio/container-yard-frontend:0.1.0
    restart: always
    ports:
      - "<frontend-port>:3000"
    environment:
      - NUXT_PUBLIC_API_URL="<backend-url>"

  backend:
    image: ghcr.io/tobinio/container-yard-backend:0.1.0
    restart: always
    ports:
      - "<backend-port>:8080"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock # access to the docker.sock outside of the host
      - $HOME/.docker:/root/.docker # access to the login data of the host
      - ../:/projects
    environment:
      - PORT=8080
      - PROJECT_DIR=/projects
      - SECRET=<jwt-secret>
      - ADMIN_NAME=<admin-username>
      - ADMIN_PASSWORD=<admin-password>
```

## Single Domain Setup

To use a single domain, we need to set up two things:

### NUXT_PUBLIC_API_URL

For this we will use our domain + `/backend-api`

For example:\
`NUXT_PUBLIC_API_URL="https://container.tobinio.dev/backend-api"`

### Proxie

The Proxie needs to understand this.\
It needs to redirect all `/backend-api` request to our backend and remove the `/backend-api` prefix.

Under Nginx this can be done by adding a `customLocation` for `/backend-api`
and configuring it like this:

```
rewrite ^/backend-api/(.*)$ /$1 break;
```
