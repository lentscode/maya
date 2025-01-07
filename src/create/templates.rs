#[derive(Clone)]
pub struct Template {
  pub name: &'static str,
  pub description: &'static str,
  pub filename: &'static str,
  pub content: &'static str,
}

pub const TEMPLATES: [Template; 3] = [
  Template {
    name: "Dockerfile Node (npm)",
    description: "Create a Dockerfile template for a Node.js project (using npm)",
    filename: "Dockerfile",
    content: "FROM node:alpine

# Set the working directory
WORKDIR /app

# Copy the package.json and package-lock.json files
# and install dependencies
COPY package.json package-lock.json ./
RUN npm install

# Copy the project files
# (comment out if you want to use a volume)
COPY . .

# Run the project
CMD [\"npm\", \"run\", \"dev\"]
    ",
  },
  Template {
    name: "Dockerfile Node (yarn)",
    description: "Create a Dockerfile template for a Node.js project (using yarn)",
    filename: "Dockerfile",
    content: "FROM node:alpine

WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install

# Copy the project files
# (comment out if you want to use a volume)
COPY . .

# Run the project
CMD [\"yarn\", \"dev\"]
    ",
  },
  Template {
    name: "Dockerfile Bun",
    description: "Create a Dockerfile template for a Bun.js project",
    filename: "Dockerfile",
    content: "FROM oven/bun:alpine

WORKDIR /app

COPY package.json bun.lockb ./
RUN bun install

# Copy the project files
# (comment out if you want to use a volume)
COPY . .

# Run the project
CMD [\"bun\", \"run\", \"dev\"]
    ",
  },
];
