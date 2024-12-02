"use client";

import Image from "next/image";
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
    <div className="relative h-screen">
      {/* Modal */}
      {vaultsShown && (
        <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10">
          <div className="relative border-0 w-[700px] h-[500px] bg-white">
            <button
              className="absolute top-4 left-4 text-white text-2xl font-bold px-4 py-2"
              onClick={() => setVaultsShown(false)}
            >
              <Image
                src="/svgviewer-png-output.png"
                alt="close"
                width={32}
                height={32}
              />
            </button>
          </div>
        </div>
      )}

      {/* Main View */}
      <div
        className={`flex flex-col items-center justify-between h-screen py-20 ${
          vaultsShown ? "bg-white/10" : ""
        } z-0`}
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
    </div>
  );
}
