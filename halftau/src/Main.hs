{-# LANGUAGE OverloadedStrings #-}
module Main where

import           Data.Array
import qualified Data.Map.Strict               as Map
import           Data.Text
import           GHC.Float
import           GHC.Int

data Elt
    = EInt(Int64)
    | EDouble(Double)
    | EBool(Bool)
    | EString(Text)
    | ESymbol(Text)
    | EList([Elt])
    | EVector(Array Int64 Elt)
    | EFn([Text], Elt)
    | EBuiltin(Builtin)
    | EMacro([Text], Elt)
    | ENil
    deriving (Show)

data Builtin
    = BPrint
    | BDef
    | BQuote
    | BFn
    | BMacro
    | BHead
    | BTail
    | BCons
    | BNo
    | BIf
    | BNot
    | BNth
    | BPlus
    | BMinus
    | BMult
    | BDiv
    | BGT
    | BEqual
    | BAssert
    | BAssertEq
    deriving (Show)

data Env = Env
    { envRootScope :: Map.Map Text Elt
    }

evalBuiltin :: Env -> Builtin -> [Elt] -> IO (Either Text (Env, Elt))
evalBuiltin env builtin args = case builtin of
    BPrint -> do
        putStrLn $ show args
        pure $ Right $ (env, ENil)
    b -> pure $ Left $ pack $ "builtin " ++ (show b) ++ " unimplemented"

evalCall :: Env -> Elt -> [Elt] -> IO (Either Text (Env, Elt))
evalCall env fn rawArgs = case fn of
    EBuiltin b -> evalBuiltin env b rawArgs
    _          -> pure $ Left "function eval unimplemented"

eval :: Env -> Elt -> IO (Either Text (Env, Elt))
eval env elt = case elt of
    EList l -> case l of
        []              -> pure $ Left "attempt to eval empty list"
        rawFn : rawArgs -> do
            evaled <- eval env rawFn

            case evaled of
                Right (env', fn) -> evalCall env' fn rawArgs
                l                -> pure l

    ESymbol s -> case Map.lookup s $ envRootScope env of
        Just e  -> pure $ Right (env, e)
        Nothing -> pure $ Left $ pack $ "undefined reference to " ++ show s

    _ -> pure $ Right (env, elt)


nextElt :: Text -> Either Text (Elt, Text)
nextElt t = Right (ENil, "")

defaultEnv :: Env
defaultEnv = Env
    { envRootScope = Map.fromList
                         [ ("print"    , EBuiltin BPrint)
                         , ("def"      , EBuiltin BDef)
                         , ("quote"    , EBuiltin BQuote)
                         , ("fn"       , EBuiltin BFn)
                         , ("macro"    , EBuiltin BMacro)
                         , ("head"     , EBuiltin BHead)
                         , ("tail"     , EBuiltin BTail)
                         , ("cons"     , EBuiltin BCons)
                         , ("no"       , EBuiltin BNo)
                         , ("if"       , EBuiltin BIf)
                         , ("not"      , EBuiltin BNot)
                         , ("nth"      , EBuiltin BNth)
                         , ("plus"     , EBuiltin BPlus)
                         , ("minus"    , EBuiltin BMinus)
                         , ("mult"     , EBuiltin BMult)
                         , ("div"      , EBuiltin BDiv)
                         , (">"        , EBuiltin BGT)
                         , ("="        , EBuiltin BEqual)
                         , ("assert"   , EBuiltin BAssert)
                         , ("assert-eq", EBuiltin BAssertEq)
                         ]
    }

main :: IO ()
main = do
    v <- eval defaultEnv (EList [ESymbol "print", ENil])
    case v of
        Right (env', v') -> pure ()
        Left  e          -> putStrLn $ "executing error: " ++ show e
