import { cn } from '@/lib/utils'

interface LogoProps {
  className?: string
  showText?: boolean
  size?: 'sm' | 'md' | 'lg'
}

export function Logo({ className, showText = true, size = 'md' }: LogoProps) {
  const sizeClasses = {
    sm: 'w-6 h-6',
    md: 'w-8 h-8',
    lg: 'w-10 h-10',
  }

  const textClasses = {
    sm: 'text-lg',
    md: 'text-xl',
    lg: 'text-2xl',
  }

  return (
    <div
      className={cn(
        'flex items-center gap-2 text-slate-900 dark:text-slate-100',
        className
      )}
    >
      <div
        className={cn(
          'flex items-center justify-center bg-primary rounded-lg text-white',
          sizeClasses[size]
        )}
      >
        <svg
          className="w-5 h-5"
          fill="none"
          viewBox="0 0 48 48"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M24 4C25.7818 14.2173 33.7827 22.2182 44 24C33.7827 25.7818 25.7818 33.7827 24 44C22.2182 33.7827 14.2173 25.7818 4 24C14.2173 22.2182 22.2182 14.2173 24 4Z"
            fill="currentColor"
          />
        </svg>
      </div>
      {showText && (
        <h1 className={cn('font-bold tracking-tight', textClasses[size])}>
          finn-lens
        </h1>
      )}
    </div>
  )
}
