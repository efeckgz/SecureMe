import Button from "./common/button";
import { useState } from "react";
import { Check, X } from "@geist-ui/icons";
import { invoke } from "@tauri-apps/api/core";

const CheckPassword = ({
  closeFunc,
  path,
}: {
  closeFunc: () => void;
  path: string;
}) => {
  const [verifyPassField, setVerifyPassField] = useState("");
  const [showIncorrectPass, setShowIncorrectPass] = useState(false);

  return (
    <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center z-20 bg-white/10">
      <form className="flex flex-col relative w-[400px] h-[147px] bg-black rounded-lg">
        {showIncorrectPass ?? <IncorrectPassword />}
        <div className="flex flex-col top-4 left-4 pt-4 px-4">
          <h1 className="text-2xl font-bold">Enter password</h1>
          <input
            className="shadow appearance-none border border-white/20 bg-black rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline text-xl mt-2"
            id="password"
            type="password"
            placeholder="Password"
            onChange={(e) => setVerifyPassField(e.target.value)}
          />
        </div>
        <div className="flex flex-row justify-end items-center w-full py-1 px-4">
          <Button
            onClick={async () => {
              console.log("Check password");
              invoke("unlock_vault", {
                path: path,
                password: verifyPassField,
              })
                .catch((e) => setShowIncorrectPass(true))
                .finally(() => {
                  closeFunc();
                });
            }}
          >
            <Check />
          </Button>
          <Button
            onClick={() => {
              console.log("Close check modal");
              closeFunc();
            }}
          >
            <X />
          </Button>
        </div>
      </form>
    </div>
  );
};

const IncorrectPassword: React.FC = () => {
  return <div>helo</div>;
};

export default CheckPassword;
