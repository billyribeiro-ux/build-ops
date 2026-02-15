export interface NavItem {
  label: string;
  href: string;
  icon: string;
  shortcut?: string;
}

export const mainNavItems: NavItem[] = [
  { label: 'Dashboard', href: '/', icon: 'ph:house-bold', shortcut: '⌘1' },
  { label: 'Programs', href: '/programs', icon: 'ph:books-bold', shortcut: '⌘2' },
  { label: 'Analytics', href: '/analytics', icon: 'ph:chart-line-bold', shortcut: '⌘3' },
  { label: 'Reviews', href: '/reviews', icon: 'ph:calendar-check-bold', shortcut: '⌘4' },
  { label: 'Evidence', href: '/evidence', icon: 'ph:archive-bold', shortcut: '⌘5' },
  { label: 'Search', href: '/search', icon: 'ph:magnifying-glass-bold', shortcut: '⌘K' },
  { label: 'Export', href: '/export', icon: 'ph:export-bold' },
];

export const bottomNavItems: NavItem[] = [
  { label: 'Import', href: '/import', icon: 'ph:upload-bold' },
  { label: 'Settings', href: '/settings', icon: 'ph:gear-bold' },
];
