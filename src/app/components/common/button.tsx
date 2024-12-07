const Button = ({
  children,
  onClick,
}: {
  children: React.ReactNode;
  onClick: () => void;
}) => {
  return (
    <button className="p-2 rounded hover:bg-white/10" onClick={() => onClick()}>
      {children}
    </button>
  );
};

export default Button;
