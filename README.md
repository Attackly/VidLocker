# VidLocker
## Description
VidLocker is a full-stack web application that allows users to download and manage videos efficiently.
The backend is built using Rust, while the frontend is written in SvelteKit.

## Why
It was mainly for VTubers. After the "Termination" of Selen Tatsuki.
It aims at Preserving the Livestreams and videos of talents.

## Features (as of now)
- Video download
- Dockerized deployment for easy setup.

## Prerequisites
- Docker and Docker Compose
- Node.js (for local usage/development)
- Rust (for local usage/development)
- yt-dlp (for local usage/development)

## Setup Instructions
### Docker (recomended)
Copy the docker-compose.yaml
Fill out the env. variables.

Fire it up.
#### Local
Local Development (Optional)


#### a. Frontend
1. Navigate to the frontend directory:
   ```bash
   $ cd frontend
   ```
2. Install dependencies:
   ```bash
   $ npm install
   ```
3. Build the frontend:
   ```bash
   $ npm run build
   ```

#### b. Backend
1. Navigate to the backend directory:
   ```bash
   $ cd backend
   ```
2. Run the backend:
   ```bash
   $ cargo build
   ```
3. Start the development server:
   ```bash
   $ cargo run
   ```

## License
This project is licensed under the AGPLv3 License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Please follow these steps:
1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Commit your changes with a short message of what it does.
4. Open a pull request branch.

## Problems, features, etc.
For questions or help open an issue.
Or Contact me on Discord. @Attackly
