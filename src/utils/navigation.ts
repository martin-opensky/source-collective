import { SettingsIcon, SourceIcon, StoryIcon } from './icons'

export interface INavigationItem {
  name: string
  href: string
  icon: React.ComponentType<{ className?: string }>
  secondaryMobileMenu: boolean // If a secondary menu should be available on mobile
  closeMobileMenu?: boolean // If the mobile menu should be closed after clicking on the navigation item
}

export const navigationItems: INavigationItem[] = [
  {
    name: 'Sources',
    href: '/sources',
    icon: SourceIcon,
    secondaryMobileMenu: true,
  },
  {
    name: 'Stories',
    href: '/stories',
    icon: StoryIcon,
    secondaryMobileMenu: true,
  },
  {
    name: 'Settings',
    href: '/settings',
    icon: SettingsIcon,
    secondaryMobileMenu: true,
  },
]
