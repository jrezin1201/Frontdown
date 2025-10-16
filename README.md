<div align="center">
  <img src="docs/assets/logo.svg" width="200" alt="RavensOne Logo">
  
  # RavensOne
  
  **One language. One stack. Ship faster.**
  
  A unified programming language for full-stack web development,
  designed for AI-assisted coding.
  
  [![CI](https://github.com/yourusername/ravensone/workflows/CI/badge.svg)](https://github.com/yourusername/ravensone/actions)
  [![Crates.io](https://img.shields.io/crates/v/ravensone.svg)](https://crates.io/crates/ravensone)
  [![Documentation](https://img.shields.io/badge/docs-ravensone.dev-blue)](https://ravensone.dev)
  [![Discord](https://img.shields.io/discord/xxxxx)](https://discord.gg/ravensone)
  
  [Website](https://ravensone.dev) •
  [Documentation](https://ravensone.dev/docs) •
  [Examples](https://github.com/yourusername/ravensone/tree/main/examples) •
  [Playground](https://ravensone.dev/playground)
</div>

---

## ✨ Features

- 🎯 **Unified Language** - Components, API routes, database schema in one file
- 🔒 **Type-Safe** - End-to-end type safety from database to UI
- ⚡ **Fast** - Compiles to optimized TypeScript/React
- 🤖 **AI-Optimized** - Designed for Claude and other AI assistants
- 🎨 **Batteries Included** - No configuration needed
- 🚀 **Deploy Anywhere** - Vercel, Netlify, Docker, Cloudflare

## 🚀 Quick Start

### Installation

```bash
# Install via Cargo
cargo install ravensone

# Or download pre-built binary
curl -fsSL https://ravensone.dev/install.sh | sh
```

### Create Your First Project

```bash
# Initialize new project
raven init my-app
cd my-app

# Start development server
raven dev
```

### Write Your First Component

```raven
// src/components/Button.raven
component Button(text: string, onClick: fn() -> void) {
  return (
    <button 
      class="px-4 py-2 bg-blue-600 text-white rounded"
      on:click={onClick}
    >
      {text}
    </button>
  )
}
```

### Add a Server Function

```raven
// src/lib/users.raven
server fn getUser(id: uuid) -> Result<User, Error> {
  user = db.users.findUnique({ where: { id } })
  return Ok(user)
}
```

### Build & Deploy

```bash
cargo build
cargo test
cargo run -- build examples/01-hello-world/src/app.raven -o dist/app.tsx
```

## 📖 Documentation

- [Getting Started](https://ravensone.dev/docs/getting-started)
- [Language Guide](https://ravensone.dev/docs/language)
- [API Reference](https://ravensone.dev/docs/api)
- [Examples](https://github.com/yourusername/ravensone/tree/main/examples)

## 🎓 Examples

|Example                                     |Description       |Complexity|
|--------------------------------------------|------------------|----------|
|[Hello World](examples/01-hello-world)      |Basic component   |⭐         |
|[Todo App](examples/02-todo-app)            |CRUD operations   |⭐⭐        |
|[Blog](examples/03-blog)                    |Posts & comments  |⭐⭐⭐       |
|[Team Dashboard](examples/04-team-dashboard)|Full SaaS app     |⭐⭐⭐⭐      |
|[E-commerce](examples/05-ecommerce)         |Shop with payments|⭐⭐⭐⭐⭐     |

## 🤝 Contributing

We welcome contributions! See <CONTRIBUTING.md> for details.

```bash
# Setup development environment
git clone https://github.com/yourusername/ravensone.git
cd ravensone
cargo build

# Run tests
cargo test

# Format code
cargo fmt
```

## 📊 Project Status

- ✅ Core compiler complete
- ✅ Type system working
- ✅ Code generation functional
- 🚧 Standard library (80% complete)
- 🚧 Dev server (functional, needs polish)
- 📋 VSCode extension (planned)
- 📋 Package manager (planned)

## 🗺️ Roadmap

See <ROADMAP.md> for detailed plans.

**Q1 2025:** Stability & testing  
**Q2 2025:** Developer experience  
**Q3 2025:** Ecosystem growth  
**Q4 2025:** Enterprise features

## 📜 License

MIT License - see <LICENSE> for details.

## 🙏 Acknowledgments

Built with ❤️ by developers who want to ship faster.

Special thanks to:

- The Rust community
- TypeScript, React, Prisma teams
- Anthropic for Claude

## 💬 Community

- [Discord](https://discord.gg/ravensone)
- [Twitter](https://twitter.com/ravensone)
- [Reddit](https://reddit.com/r/ravensone)

-----

<div align="center">
  <sub>Built by <a href="https://github.com/yourusername">@yourusername</a></sub>
</div>
