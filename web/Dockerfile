    # Use the official Node.js 16 image as the base image
    FROM node:latest

    # Set the working directory to /app
    WORKDIR /app

    # Copy the package.json and package-lock.json files to the container
    COPY package*.json ./

    # Install dependencies using pnpm
    RUN npm install -g pnpm && pnpm install

    # Copy the rest of the application code to the container
    COPY . .

    # Build the application using pnpm run build
    RUN pnpm run build

    # Install a simple HTTP server to serve the built app
    RUN pnpm install -g http-server

    # Expose the port on which the server will run
    EXPOSE 8080

    # Start the HTTP server to serve the built app
CMD ["http-server", "dist", "-p", "8080", "-d", "false"]
