# Units POC - Next.js Frontend

This is a proof-of-concept frontend application for the Units project, built with Next.js and TypeScript. It provides a user interface for managing token handlers, user onboarding, and program execution through gRPC-Web services.

## Features

- Load and manage token handlers
- User onboarding through token binding
- Program execution interface
- Real-time JSON response visualization
- Modern UI components using shadcn/ui
- gRPC-Web integration for backend communication

## Prerequisites

- Node.js >= 18.17.0
- npm or yarn
- Running backend gRPC server (expected at `localhost:8080`)

## Getting Started

1. Clone the repository:
```bash
git clone <repository-url>
cd my-finternet-app
```

2. Install dependencies:
```bash
npm install
# or
yarn install
```

3. Run the development server:
```bash
npm run dev
# or
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

## Project Structure

```
src/
├── app/                    # Next.js app directory
├── components/             # React components
│   ├── ui/                # Reusable UI components
│   ├── BindForm.tsx       # User onboarding form
│   ├── ExecuteForm.tsx    # Program execution form
│   └── LoadDriverForm.tsx # Token handler form
├── lib/                   # Utility functions and backend services
└── proto/                 # Generated gRPC-Web code
```

## Key Components

### Token Handler Management
- Upload and configure token handlers
- Support for both WAT and WASM binary types
- Version management

### User Onboarding
- Token binding interface
- Account information configuration
- Path management

### Program Execution
- Execute WebAssembly programs
- Real-time feedback
- JSON input/output handling

## Technology Stack

- **Framework**: Next.js 15.1
- **Language**: TypeScript
- **Styling**: TailwindCSS
- **UI Components**: shadcn/ui
- **Backend Communication**: gRPC-Web
- **State Management**: React Hooks
- **Form Handling**: Native React forms
- **Icons**: Lucide React

## API Integration

The application integrates with three main gRPC services:
- Driver Service: For token handler management
- Bind Service: For user onboarding
- Execution Service: For program execution

All services communicate through gRPC-Web protocols with the backend server.

## Development

### Adding New Components

1. Create new components in the `src/components` directory
2. Use shadcn/ui components from `src/components/ui` for consistent styling
3. Implement gRPC-Web service calls through `src/lib/backend.ts`

### Styling

- Uses TailwindCSS for styling
- Custom theme configuration in `tailwind.config.ts`
- Dark mode support through CSS variables
- shadcn/ui component styling system

### Backend Communication

All backend service calls are centralized in `src/lib/backend.ts`. To add new service calls:

1. Define new methods in the backend service file
2. Use the appropriate gRPC-Web client
3. Handle responses through the JSON prettifier component

## Building for Production

```bash
npm run build
# or
yarn build
```

The built application will be in the `.next` directory.

## Scripts

- `dev`: Run development server
- `build`: Build for production
- `start`: Start production server
- `lint`: Run ESLint
