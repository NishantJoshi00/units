# Finternet App

A Next.js application that enables secure onboarding and execution of WebAssembly modules through a gRPC interface.

## Features

- Load and manage token handlers (WebAssembly/WAT modules)
- User onboarding with account binding
- Execute WebAssembly programs
- Modern, responsive UI built with Tailwind CSS
- Type-safe gRPC-web communication

## Prerequisites

- Node.js 18.17.0 or later
- npm or yarn package manager
- A running gRPC server that implements the Finternet protocol

## Getting Started

1. Clone the repository:
```bash
git clone https://github.com/yourusername/my-finternet-app.git
cd my-finternet-app
```

2. Install dependencies:
```bash
npm install
# or
yarn
```

3. Start the development server:
```bash
npm run dev
# or
yarn dev
```

4. Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

## Project Structure

```
src/
├── app/                 # Next.js app router
├── components/          # React components
│   ├── ui/             # UI components (buttons, inputs, etc.)
│   └── ...             # Feature components
├── lib/                # Utility functions and backend API
└── proto/              # Protocol buffer definitions
```

## Key Technologies

- [Next.js](https://nextjs.org/) - React framework
- [Tailwind CSS](https://tailwindcss.com/) - CSS framework
- [shadcn/ui](https://ui.shadcn.com/) - UI component library
- [gRPC-web](https://github.com/grpc/grpc-web) - gRPC for web clients
- [Protocol Buffers](https://protobuf.dev/) - Data serialization format
- [Radix UI](https://www.radix-ui.com/) - Primitive UI components

## Development

### API Integration

The application communicates with a gRPC server using the following services:

- `Driver` - For managing token handlers
- `Bind` - For user onboarding
- `Execution` - For running WebAssembly programs

The Protocol Buffer definitions can be found in `src/proto/finternet.proto`.

### Adding New Components

1. Create new components in the `src/components` directory
2. For UI components, use the `src/components/ui` directory
3. Follow the existing component patterns and TypeScript types

### Styling

- Use Tailwind CSS classes for styling
- Maintain the dark/light theme compatibility
- Follow the color scheme defined in `tailwind.config.ts`

## Building for Production

```bash
npm run build
# or
yarn build
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
