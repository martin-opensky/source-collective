import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import classNames from 'classnames'
import './globals.css'
import { ReduxProvider } from '@/store/Provider'
import DesktopNavigation from '@/components/Navigation/Desktop'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Source Collective',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body
        className={classNames({
          [inter.className]: true,
          // 'h-screen w-screen': true,
        })}
      >
        <ReduxProvider>
          <main className="max-h-mobile min-h-mobile flex flex-row lg:min-h-screen">
            <DesktopNavigation />

            <div className="max-h-mobile min-h-mobile flex flex-1 flex-col overflow-scroll lg:min-h-screen lg:pl-16">
              {children}
            </div>
          </main>
        </ReduxProvider>
      </body>
    </html>
  )
}
