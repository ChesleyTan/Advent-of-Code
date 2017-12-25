{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Maybe (isJust, fromJust)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate, delete, (\\))
import Data.List.Split (chunksOf)
import Debug.Trace (trace)
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

readInt :: String -> Int
readInt = read

data Direction = DRight | DLeft deriving (Show, Eq)

data Rule = Rule
          { ifState :: Char
          , zeroWrite :: Int
          , zeroMove :: Direction
          , zeroState :: Char
          , oneWrite :: Int
          , oneMove :: Direction
          , oneState :: Char
          } deriving (Show, Eq)

data TM = TM
        { tape :: !(M.Map Int Int)
        , cursor :: !Int
        , state :: !Char
        , rules :: [Rule]
        } deriving (Show)

parse :: [String] -> Rule
parse [inState, _, zeroCase, zeroMove, zeroState, _, oneCase, oneMove, oneState] =
    Rule
    { ifState = inState !! 9
    , zeroWrite = [zeroCase !! 22] |> readInt
    , zeroMove = if zeroMove !! 27 == 'r' then DRight else DLeft
    , zeroState = zeroState !! 26
    , oneWrite = [oneCase !! 22] |> readInt
    , oneMove = if oneMove !! 27 == 'r' then DRight else DLeft
    , oneState = oneState !! 26
    }

matchRule :: Char -> Rule -> Bool
matchRule currentState rule = ifState rule == currentState

findRule :: [Rule] -> Char -> Rule
findRule rules currentState = head $ filter (matchRule currentState) rules

transition :: TM -> TM
transition !tm =
    let matchingRule = findRule (rules tm) (state tm)
        currentValue = M.findWithDefault 0 (cursor tm) (tape tm)
        newValue = if currentValue == 0 then zeroWrite matchingRule else oneWrite matchingRule
        moveDirection = if currentValue == 0 then zeroMove matchingRule else oneMove matchingRule
        newState = if currentValue == 0 then zeroState matchingRule else oneState matchingRule
        newCursor = if moveDirection == DRight then cursor tm + 1 else cursor tm - 1
        newTape = M.insert (cursor tm) newValue (tape tm)
    in tm { tape = newTape
          , cursor = newCursor
          , state = newState
          }

main :: IO ()
main = do
    input <- T.lines <$> T.readFile "25a.input"
    let startState = 'A'
        checksumAfter = 12481997
        rules = input |> drop 2 |> map T.unpack |> filter (not . null) |> chunksOf 9 |> map parse
        startingTm = TM { tape = M.empty
                        , cursor = 0
                        , state = startState
                        , rules = rules
                        }
        tmStates = iterate transition startingTm
        checksumState = tmStates !! checksumAfter
        checksum = M.foldr (+) 0 (tape checksumState)
    print checksum
