"use client";

import { useState } from "react";
import { X, Trash2 } from "@geist-ui/icons";

interface MenuButtonProps {
  title: string;
  action: () => void;
}

interface Vault {
  name: string;
  path: string;
  isLocked: boolean;
  lastAccessed: Date;
  created: Date;
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
  //border-b border-white/10
  return (
    <div className="relative h-screen">
      {/* Modal */}
      {vaultsShown && (
        <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10 bg-white/10">
          <div className="relative w-[700px] h-[500px] bg-black text-white rounded-lg">
            {/* Header Section */}
            <div className="flex justify-between items-center px-4 py-4 border-b border-white/10">
              <div className="flex items-center space-x-4">
                <button
                  className="p-2 rounded hover:bg-white/10"
                  onClick={() => setVaultsShown(false)}
                  aria-label="Close vaults modal"
                >
                  <X />
                </button>
                <h1 className="font-bold text-lg">Vault</h1>
              </div>
              <div className="flex space-x-4 text-lg font-bold">
                <span>Created</span>
                <span>Last accessed</span>
              </div>
            </div>

            {/* Modal Content */}
            <div className="p-4 overflow-y-auto h-[calc(100%-4rem)]">
              {/* <p className="text-center text-gray-400">Your vault content...</p> */}
              <div className="space-y-4">
                {Array.from({ length: 50 }).map((_, i) => (
                  <div
                    key={i}
                    className="flex items-center justify-between border-b border-white/10 pb-2"
                  >
                    {/* Delete Icon */}
                    <button
                      className="p-2 rounded hover:bg-white/10"
                      aria-label="Delete vault"
                    >
                      <Trash2 />
                    </button>

                    {/* Vault Name */}
                    <span className="flex-1 pl-4">Vault #{i + 1}</span>

                    {/* Created Date */}
                    <span className="px-4 text-sm text-gray-300">
                      2024-12-01
                    </span>

                    {/* Last Accessed Date */}
                    <span className="px-4 text-sm text-gray-300">
                      2024-12-02
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      )}

      {/* {vaultsShown && (
        <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10">
          <div className="relative w-[700px] h-[500px] bg-black">
            <div className="flex flex-row absolute inset-x-0 top-0 h-16">
              <div className="absolute top-4 left-4 px-4 py-2 flex justify-between w-full font-bold text-lg">
                <div className="flex">
                  <button onClick={() => setVaultsShown(false)}>
                    <X />
                  </button>
                  <h1 className="px-16">Vault</h1>
                </div>
                <div className="flex px-4">
                  <h1 className="px-4">Created</h1>
                  <h1 className="px-4">Last accessed</h1>
                </div>
              </div>
            </div>
          </div>
        </div>
      )} */}

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

// const ModalSection = (icon: React.ReactNode, )
