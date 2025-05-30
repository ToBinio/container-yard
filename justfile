publish frontend-version backend-version:
    just --justfile ./frontend/justfile publish {{frontend-version}}
    just --justfile ./backend/justfile publish {{backend-version}}
