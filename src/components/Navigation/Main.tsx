import Image from 'next/image'
import Link from 'next/link'
import { navigationItems } from '@/utils/navigation'
import NavigationItem from './NavigationItem'

export default function Main() {
  return (
    <>
      <Link href="/">
        <Image
          src="/images/source-collective.png"
          alt="Logo"
          className="mx-auto my-1 h-auto cursor-pointer p-2"
          width={128}
          height={128}
          priority
        />
      </Link>
      <div className="flex flex-grow flex-col items-center justify-between">
        <nav className="flex flex-1 flex-col">
          <ul role="list" className="mx-1 mt-2 flex flex-col gap-2">
            {navigationItems.map((item) => (
              <NavigationItem key={item.name} item={item} />
            ))}
          </ul>
        </nav>

        {/* <UserInfo className="hidden lg:block" /> */}
      </div>
    </>
  )
}
