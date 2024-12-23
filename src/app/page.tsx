"use client";

import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import SelectVaults from "./components/selectVaults";
import EnterCredentials from "./components/enterCredentials";
import { useModal } from "./hooks/useModal";

interface MenuButtonProps {
  title: string;
  action: () => void;
}

export default function Home() {
  // TODO: Share this state with enterCredentials
  const [userVaultDir, setUserVaultDir] = useState("");

  const { isOpen: enterCredentialIsOpen, open: openEnterCredentials } =
    useModal("enterCredentials");

  const { isOpen: vaultsIsOpen, open: openVaults } = useModal("vaults");

  const buttons: MenuButtonProps[] = [
    {
      title: "Create new vault",
      action: async () => {
        const dir = await open({
          multiple: false,
          directory: true,
        });

        if (dir) {
          setUserVaultDir(dir);
          openEnterCredentials();
        }
      },
    },
    {
      title: "Manage vaults",
      action: () => {
        openVaults();
      },
    },
  ];

  return (
    <div className="relative h-screen">
      {enterCredentialIsOpen && (
        <EnterCredentials
          userVaultDir={userVaultDir} // Pass the dir to the confirmation modal
        />
      )}

      {vaultsIsOpen && <SelectVaults />}

      {/* Main View */}
      <div
        className={`flex flex-col items-center justify-between h-screen py-20 ${
          vaultsIsOpen ? "bg-white/10" : ""
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
                className="rounded-lg bg-white/20"
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
