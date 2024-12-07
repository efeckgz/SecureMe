"use client";

import { useState } from "react";
import { X, Check } from "@geist-ui/icons";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

import SelectVaults from "./components/selectVaults";
import EnterCredentials from "./components/enterCredentials";
import Button from "./components/common/button";

export default function Home() {
  const [vaultsShown, setVaultsShown] = useState(false);
  const [credentialScreenShown, setCredentialScreenShown] = useState(false);

  // TODO: Share this state with enterCredentials
  const [userVaultDir, setUserVaultDir] = useState("");

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
        }

        setCredentialScreenShown(true);

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
      action: async () => {
        console.log("Delete vault");
      },
    },
  ];

  return (
    <div className="relative h-screen">
      {/* Enter vault name & password */}
      {credentialScreenShown && (
        <EnterCredentials
          userVaultDir={userVaultDir} // Pass the dir to the confirmation modal
          closeFunc={() => setCredentialScreenShown(false)}
        />
      )}
      {/* Modal */}
      {vaultsShown && <SelectVaults closeFunc={() => setVaultsShown(false)} />}

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
