"use client";

interface MenuButtonProps {
  title: string;
  action: () => void;
}

const buttons: MenuButtonProps[] = [
  {
    title: "Create new vault",
    action: () => {
      console.log("Created new vault!");
    },
  },
  {
    title: "Open a vault",
    action: () => {
      console.log("Opened a vault!");
    },
  },
  {
    title: "Delete a vault",
    action: () => {
      console.log("Deleted a vault!");
    },
  },
];

export default function Home() {
  return (
    <div className="flex flex-col items-center justify-between h-screen py-20">
      <div className="flex flex-col items-center">
        <h1 className="text-6xl text-indigo-900 font-bold">
          Welcome to SecureMe.
        </h1>
        <p className="text-black">Please start by selecting an option below.</p>
      </div>
      <div className="flex flex-col space-y-2">
        {buttons.map(({ title, action }, index) => {
          return (
            <button
              className="border-0 rounded-lg bg-black/20"
              onClick={action}
              key={index}
            >
              <div className="my-2 mx-12 text-black">{title}</div>
            </button>
          );
        })}
      </div>
    </div>
  );
}
