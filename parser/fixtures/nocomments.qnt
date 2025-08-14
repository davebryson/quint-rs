module tictactoe {
  type Player = X | O
  type Square = Occupied(Player) | Empty

  var board: int -> (int -> Square)

  var nextTurn: Player

  pure val boardCoordinates = tuples(1.to(3), 1.to(3))

  pure val winningPatterns = Set(
    Set((1,1), (1,2), (1,3)),
    Set((2,1), (2,2), (2,3)),
    Set((3,1), (3,2), (3,3)),
    Set((1,1), (2,1), (3,1)),
    Set((1,2), (2,2), (3,2)),
    Set((1,3), (2,3), (3,3)),
    Set((1,1), (2,2), (3,3)),
    Set((3,1), (2,2), (1,3))
  )

  pure val corners = Set(
    (1,1),
    (3,1),
    (1,3),
    (3,3)
  )

  def square(coordinate: (int, int)): Square =
    board.get(coordinate._1).get(coordinate._2)

  def hasPlayer(coordinate, player) = match square(coordinate) {
    | Empty   => false
    | Occupied(p) => player == p
  }

  def isEmpty(coordinate) = match square(coordinate) {
    | Empty  => true
    | _      => false
  }

  val boardEmpty = boardCoordinates.forall(isEmpty)

  def won(player) = winningPatterns.exists(pattern =>
    pattern.forall(coordinate => hasPlayer(coordinate, player))
  )

  val boardFull = not(boardCoordinates.exists(isEmpty))
  val stalemate = boardFull and not(won(X)) and not(won(O))
  val gameOver = won(X) or won(O) or boardFull

  def canWinWithPattern(pattern) = and {
    pattern.filter(coordinate => coordinate.hasPlayer(X)).size() == 2,
    pattern.filter(coordinate => coordinate.isEmpty()).size() == 1,
  }

  def canBlockWithPattern(pattern) = and {
    pattern.filter(coordinate => coordinate.hasPlayer(O)).size() == 2,
    pattern.filter(coordinate => coordinate.isEmpty()).size() == 1,
  }

  def canSetupWinWithPattern(pattern) = and {
    pattern.filter(coordinate => coordinate.hasPlayer(X)).size() == 1,
    pattern.filter(coordinate => coordinate.isEmpty()).size() == 2,
  }

  val canWin = winningPatterns.exists(canWinWithPattern)
  val canBlock = winningPatterns.exists(canBlockWithPattern)
  val canTakeCenter = isEmpty((2,2))
  val canSetupWin = winningPatterns.exists(canSetupWinWithPattern)

  action Move(player, coordinate) = all {
    isEmpty(coordinate),
    board' = board.setBy(
      coordinate._1,
      row => row.set(coordinate._2, Occupied(player))
    ),
  }

  action Win = all {
    canWin,
    nondet pattern = winningPatterns.filter(canWinWithPattern).oneOf()
    nondet coordinate = pattern.filter(isEmpty).oneOf()
    Move(X, coordinate),
  }

  action Block = all {
    canBlock,
    nondet pattern = winningPatterns.filter(canBlockWithPattern).oneOf()
    nondet coordinate = pattern.filter(isEmpty).oneOf()
    Move(X, coordinate),
  }

  action TakeCenter = Move(X, (2, 2))

  action SetupWin = all {
    nondet pattern = winningPatterns.filter(canSetupWinWithPattern).oneOf()
    nondet coordinate = pattern.filter(isEmpty).oneOf()
    Move(X, coordinate),
  }

  action MoveToEmpty(player) =
    nondet coordinate = boardCoordinates.filter(isEmpty).oneOf()
    Move(player, coordinate)

  action StartInCorner =
    nondet corner = oneOf(corners)
    Move(X, corner)

  action MoveX = all {
    nextTurn == X,
    not(gameOver),
    if (boardEmpty) StartInCorner else
    if (canWin) Win else
    if (canBlock) Block else
    if (canTakeCenter) TakeCenter else
    if (canSetupWin) SetupWin else
    MoveToEmpty(X),
    nextTurn' = O,
  }

  action MoveO = all {
    nextTurn == O,
    not(gameOver),
    MoveToEmpty(O),
    nextTurn' = X,
  }

  action init = all {
    nextTurn' = X,
    board' = 1.to(3).mapBy(_ => 1.to(3).mapBy(_ => Empty)),
  }

  action step = any {
    MoveX,
    MoveO,
    all { gameOver, board' = board, nextTurn' = nextTurn },
  }


  val XHasNotWon = not(won(X))

  val OHasNotWon = not(won(O))

  val NotStalemate = not(stalemate)

  val inv = OHasNotWon

  temporal XMustEventuallyWin = eventually(won(X))
}
