import Control.Applicative ((*>), (<$>), (<*>), (<|>))
import Data.Either (rights)
import Data.Maybe (fromMaybe)
import Data.Tuple (swap)
import Text.Parsec
import Text.Parsec.Char (digit, letter, spaces, string)
import Text.Parsec.Combinator (eof, many1, sepBy)
import Text.Parsec.String (Parser)

data Round = Round
  { red :: Int,
    green :: Int,
    blue :: Int
  }
  deriving (Show)

data Game = Game
  { index :: Int,
    rounds :: [Round]
  }
  deriving (Show)

parseInt :: Parser Int
parseInt = read <$> many1 digit

parseColor :: Parser (String, Int)
parseColor = do
  num <- parseInt
  spaces
  color <- many1 letter
  return (color, num)

parseRound :: Parser Round
parseRound = do
  colors <- sepBy parseColor (string ", ")
  let red = sum [v | ("red", v) <- colors]
  let green = sum [v | ("green", v) <- colors]
  let blue = sum [v | ("blue", v) <- colors]
  return (Round red green blue)

parseGame :: Parser Game
parseGame =
  Game
    <$> (string "Game " *> parseInt <* string ": ")
    <*> sepBy parseRound (string "; ")

valid :: Game -> Bool
valid (Game index rounds) = all (\r -> red r <= 12 && green r <= 13 && blue r <= 14) rounds

getGames :: String -> [Game]
getGames content = rights . map (parse parseGame "") $ lines content

solveA :: String -> Int
solveA = sum . map index . filter valid . getGames

power :: Game -> Int
power (Game index rounds) = red bestRound * blue bestRound * green bestRound
  where
    bestRound =
      foldl1
        ( \r s ->
            Round
              (max (red r) (red s))
              (max (green r) (green s))
              (max (blue r) (blue s))
        )
        rounds

solveB :: String -> Int
solveB = sum . map power . getGames

main :: IO ()
main = do
  content <- readFile "../input/day2.in"
  print $ solveA content
  print $ solveB content
