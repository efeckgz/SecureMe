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
      <h1 className="text-6xl text-indigo-900 font-bold">Locked</h1>
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
        {/* <button className="border-0 rounded-lg bg-black/20">
          <div className="my-2 mx-12 text-black">Hello</div>
        </button>
        <button className="border-0 rounded-lg bg-black/20">
          <div className="my-2 mx-12 text-black">Hello</div>
        </button> */}
      </div>
    </div>
  );
}
