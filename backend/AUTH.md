# JWT Authentication Implementation

## Overview
JWT-based authentication protecting write endpoints (create, join, contribute).

## Setup

1. Add to `.env`:
```env
JWT_SECRET=your-secret-key-change-in-production
JWT_EXPIRES_IN=7d
```

2. Install dependencies (already done):
```bash
npm install jsonwebtoken @types/jsonwebtoken
```

## Usage

### 1. Get Token
```bash
POST /api/auth/token
Content-Type: application/json

{
  "publicKey": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
}

Response:
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### 2. Use Token for Protected Endpoints
```bash
POST /api/groups
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "My Group",
  ...
}
```

## Protected Endpoints
- `POST /api/groups` - Create group
- `POST /api/groups/:id/join` - Join group
- `POST /api/groups/:id/contribute` - Make contribution

## Public Endpoints
- `GET /api/groups` - List groups
- `GET /api/groups/:id` - Get group details
- `GET /api/groups/:id/members` - Get members
- `GET /api/groups/:id/transactions` - Get transactions

## Implementation Files
- `backend/src/services/authService.ts` - JWT generation/verification
- `backend/src/middleware/auth.ts` - Auth middleware
- `backend/src/routes/auth.ts` - Token endpoint
- `backend/src/index.ts` - Auth router registration
- `backend/src/routes/groups.ts` - Protected routes
