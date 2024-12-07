interface VaultViewModel {
  name: string;
  path: string;
  isLocked: boolean;
}

interface MenuButtonProps {
  title: string;
  action: () => void;
}
