import { Router, Request, Response } from 'express'
import { AuthService } from '../services/authService'
import { z } from 'zod'

const router = Router()

const authSchema = z.object({
  publicKey: z.string().min(1)
})

// POST /api/auth/token - Generate JWT token
router.post('/token', (req: Request, res: Response) => {
  try {
    const { publicKey } = authSchema.parse(req.body)
    const token = AuthService.generateToken(publicKey)
    res.json({ token })
  } catch (error) {
    res.status(400).json({ error: 'Invalid request' })
  }
})

export const authRouter = router
