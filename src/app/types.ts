interface VaultViewModel {
  name: string;
  path: string;
  isLocked: boolean;
}

interface MenuButtonProps {
  title: string;
  action: () => void;
}

interface VaultItemProps extends VaultViewModel {
  onDelete: () => void;
  onToggleLock: (path: string) => void;
}
