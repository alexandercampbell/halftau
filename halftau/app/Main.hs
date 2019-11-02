{-# LANGUAGE OverloadedStrings #-}
module Main where

import           Halftau

main :: IO ()
main = do
    v <- eval defaultEnv (EList [ESymbol "print", ENil, ENil])
    case v of
        Right (binds, v') -> pure ()
        Left  e           -> putStrLn $ "executing error: " ++ show e
