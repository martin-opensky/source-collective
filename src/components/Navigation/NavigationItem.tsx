'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import classNames from 'classnames'
import type { INavigationItem } from '@/utils/navigation'
// import { useAppDispatch } from '@/store/hooks'
// import { setIsMenuOpen } from "@/store/global"

interface IProps {
  item: INavigationItem
}

export default function NavigationItem({ item }: IProps) {
  const pathname = usePathname()
  // const dispatch = useAppDispatch()

  return (
    <li key={item.href} className="group relative">
      <Link
        test-id={`nav-item-${item.name}`}
        href={item.href}
        className={classNames({
          'flex h-12 w-12 flex-row items-center justify-center gap-3 rounded-full text-sm text-white hover:bg-sky-700 hover:text-slate-100 hover:drop-shadow-md':
            true,
          'bg-sky-700 text-slate-100 drop-shadow-md': pathname.startsWith(
            item.href,
          ),
        })}
        onClick={() => {
          if (item.closeMobileMenu) {
            // dispatch(setIsMenuOpen(false))
          }
        }}
      >
        <item.icon className="h-6 w-6" />
        <NavTooltip
          item={item}
          pathname={pathname}
          className="hidden lg:visible"
        />
      </Link>
    </li>
  )
}

interface INavTooltipProps extends React.HTMLAttributes<HTMLSpanElement> {
  item: INavigationItem
  pathname: string
}

function NavTooltip({ pathname, item, ...props }: INavTooltipProps) {
  return (
    <span
      {...props}
      className={classNames({
        'absolute left-16 top-3 hidden w-auto rounded-lg bg-black/75 p-2 text-xs text-white':
          true,
        hidden: pathname.startsWith(item.href),
        'group-hover:block': !pathname.startsWith(item.href),
        [props.className as string]: props.className !== undefined,
      })}
    >
      {item.name}
    </span>
  )
}
