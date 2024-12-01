"use client";

import { useState } from "react";

interface MenuButtonProps {
  title: string;
  action: () => void;
}

export default function Home() {
  const [vaultsShown, setVaultsShown] = useState(false);

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
        setVaultsShown(!vaultsShown);
      },
    },
    {
      title: "Delete a vault",
      action: () => {
        console.log("Deleted a vault!");
      },
    },
  ];

  return (
    <>
      <div className="border border-lg w-600 h-400">Hello</div>
      <div
        className={`flex flex-col items-center justify-between h-screen py-20 ${
          vaultsShown ? "bg-white/10" : ""
        }`}
      >
        <div className="flex flex-col items-center">
          <h1 className="text-6xl text-white font-bold">
            Welcome to SecureMe.
          </h1>
          <p className="text-white">
            Please start by selecting an option below.
          </p>
        </div>
        <div className="flex flex-col space-y-2">
          {buttons.map(({ title, action }, index) => {
            return (
              <button
                className="border-0 rounded-lg bg-white/20"
                onClick={action}
                key={index}
              >
                <div className="my-2 mx-12 text-white">{title}</div>
              </button>
            );
          })}
        </div>
      </div>
    </>
  );
}
