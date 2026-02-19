// Issue #22: Create group card component
// Complexity: Trivial (100 pts)
// Status: Enhanced with variants and consistent styling

import React from 'react'

interface GroupCardProps {
  groupId: string
  groupName: string
  memberCount: number
  maxMembers: number
  nextPayout: string
  totalContributions: number
  status: 'active' | 'completed' | 'paused'
  variant?: 'default' | 'elevated' | 'outlined' | 'interactive' | 'compact' | 'spacious'
  onClick?: () => void
}

export const GroupCard: React.FC<GroupCardProps> = ({
  groupName,
  memberCount,
  maxMembers,
  nextPayout,
  totalContributions,
  status,
  variant = 'interactive',
  onClick,
}) => {
  const statusColors = {
    active: 'bg-green-100 text-green-800 border-green-200',
    completed: 'bg-blue-100 text-blue-800 border-blue-200',
    paused: 'bg-yellow-100 text-yellow-800 border-yellow-200',
  }

  const cardVariants = {
    default: 'card-default',
    elevated: 'card-elevated',
    outlined: 'card-outlined',
    interactive: 'card-interactive',
    compact: 'card-compact',
    spacious: 'card-spacious',
  }

  const cardClass = cardVariants[variant]
  const isCompact = variant === 'compact'
  const isSpaciousOrElevated = variant === 'spacious' || variant === 'elevated'

  return (
    <div 
      className={cardClass}
      onClick={onClick}
      role={onClick ? 'button' : undefined}
      tabIndex={onClick ? 0 : undefined}
      onKeyDown={onClick ? (e) => e.key === 'Enter' && onClick() : undefined}
    >
      <div className={`flex justify-between items-start ${isCompact ? 'mb-3' : 'mb-4'}`}>
        <h3 className={`font-bold text-gray-900 ${isCompact ? 'text-lg' : isSpaciousOrElevated ? 'text-2xl' : 'text-xl'}`}>
          {groupName}
        </h3>
        <span 
          className={`px-3 py-1 rounded-full text-xs font-semibold border ${statusColors[status]}`}
        >
          {status.charAt(0).toUpperCase() + status.slice(1)}
        </span>
      </div>

      <div className={`card-body ${isCompact ? 'space-y-2' : 'space-y-3'}`}>
        <div className="flex justify-between items-center">
          <span className={`text-gray-600 ${isCompact ? 'text-sm' : 'text-base'}`}>Members</span>
          <span className={`font-semibold ${isCompact ? 'text-sm' : 'text-base'}`}>
            {memberCount}/{maxMembers}
          </span>
        </div>

        <div className="w-full bg-gray-200 rounded-full h-2 overflow-hidden">
          <div
            className="bg-blue-600 h-2 rounded-full transition-all duration-300 ease-out"
            style={{ width: `${(memberCount / maxMembers) * 100}%` }}
          />
        </div>

        <div className={`flex justify-between items-center ${isCompact ? 'text-xs' : 'text-sm'}`}>
          <span className="text-gray-600">Total Contributed</span>
          <span className="font-semibold text-gray-900">${totalContributions.toFixed(2)}</span>
        </div>

        <div className={`flex justify-between items-center ${isCompact ? 'text-xs' : 'text-sm'}`}>
          <span className="text-gray-600">Next Payout</span>
          <span className="text-blue-600 font-semibold">{nextPayout}</span>
        </div>
      </div>

      <button 
        className={`w-full bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white font-semibold rounded-lg transition-colors duration-200 shadow-sm hover:shadow-md ${
          isCompact ? 'mt-3 py-1.5 text-sm' : 'mt-4 py-2 text-base'
        }`}
        onClick={(e) => {
          e.stopPropagation()
          // Handle view details action
        }}
      >
        View Details
      </button>
    </div>
  )
}
