import * as React from 'react'
import { cn } from '@/lib/utils'

export interface CheckboxProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string
}

const Checkbox = React.forwardRef<HTMLInputElement, CheckboxProps>(
  ({ className, label, ...props }, ref) => {
    return (
      <label className="relative flex items-center cursor-pointer group">
        <input
          type="checkbox"
          ref={ref}
          className={cn(
            'h-5 w-5 rounded border-slate-300 dark:border-slate-700 bg-slate-50 dark:bg-slate-800 text-primary focus:ring-offset-0 focus:ring-2 focus:ring-primary/20 transition-all cursor-pointer',
            className
          )}
          {...props}
        />
        {label && (
          <span className="ml-3 text-sm text-slate-600 dark:text-slate-400 group-hover:text-slate-900 dark:group-hover:text-slate-200 transition-colors">
            {label}
          </span>
        )}
      </label>
    )
  }
)
Checkbox.displayName = 'Checkbox'

export { Checkbox }
