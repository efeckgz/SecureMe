"use client";

import { useState } from "react";
import { X, Check } from "@geist-ui/icons";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

import SelectVaults from "./components/selectVaults";
import Button from "./components/common/button";

export default function Home() {
  const [vaultsShown, setVaultsShown] = useState(false);
  const [credentialScreenShown, setCredentialScreenShown] = useState(false);

  // User entered directory, name and password for the vault to be created
  const [userVaultDir, setUserVaultDir] = useState("");
  const [userVaultPassword, setUserVaultPassword] = useState("");
  const [userVaultName, setUserVaultName] = useState("");

  const getViewModel = async () => {
    let viewmodel: VaultViewModel = await invoke("get_view_model");
    console.log(viewmodel);
  };

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
        await getViewModel();
      },
    },
  ];

  return (
    <div className="relative h-screen">
      {/* Enter vault name & password */}
      {credentialScreenShown && (
        <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-10 bg-white/10">
          <form className="flex flex-col relative w-[400px] h-[250px] bg-black rounded-lg">
            <div className="flex flex-col top-4 left-4 pt-4 px-4">
              <h1 className="text-2xl font-bold">Name</h1>
              <input
                className="shadow appearance-none border border-white/20 bg-black rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline text-xl mt-2"
                id="username"
                type="text"
                placeholder="Name for the vault"
                onChange={(e) => setUserVaultName(e.target.value)}
              />
            </div>
            <div className="flex flex-col top-4 left-4 pt-4 px-4">
              <h1 className="text-2xl font-bold">Password</h1>
              <input
                className="shadow appearance-none border border-white/20 bg-black rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline text-xl mt-2"
                id="password"
                type="password"
                placeholder="Password"
                onChange={(e) => setUserVaultPassword(e.target.value)}
              />
            </div>
            <div className="flex flex-row justify-end items-center w-full py-2 px-4">
              <Button
                onClick={async () => {
                  await invoke("create_secure_vault", {
                    name: userVaultName,
                    path: userVaultDir,
                    password: userVaultPassword,
                  });
                  setCredentialScreenShown(false);
                }}
              >
                <Check />
              </Button>
              <Button onClick={() => setCredentialScreenShown(false)}>
                <X />
              </Button>
            </div>
          </form>
        </div>
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
