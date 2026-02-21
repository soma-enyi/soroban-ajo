import { Router } from 'express'
import { GroupsController } from '../controllers/groupsController'
import { webhookMiddleware } from '../middleware/webhook'
import { authMiddleware } from '../middleware/auth'

const router = Router()
const controller = new GroupsController()

// GET /api/groups - List all groups
router.get('/', controller.listGroups)

// GET /api/groups/:id - Get group by ID
router.get('/:id', controller.getGroup)

// POST /api/groups - Create new group (with webhook) - PROTECTED
router.post('/', authMiddleware, controller.createGroup, webhookMiddleware.afterGroupCreated)

// POST /api/groups/:id/join - Join a group (with webhook) - PROTECTED
router.post('/:id/join', authMiddleware, controller.joinGroup, webhookMiddleware.afterMemberJoined)

// POST /api/groups/:id/contribute - Make contribution (with webhook) - PROTECTED
router.post('/:id/contribute', authMiddleware, controller.contribute, webhookMiddleware.afterContribution)

// GET /api/groups/:id/members - Get group members
router.get('/:id/members', controller.getMembers)

// GET /api/groups/:id/transactions - Get group transactions
router.get('/:id/transactions', controller.getTransactions)

export const groupsRouter = router
