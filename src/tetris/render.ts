import { BOARD_HEIGHT, BOARD_WIDTH } from "./config";
import { Block } from "./objects/block";
import { Floor } from "./objects/floor";
import { Piece } from "./objects/piece";
import { BlockType, BoardType } from "./types";

export function render(piece: Piece, floor: Floor, projectedPiece: Piece) {
  const board = getEmptyBoard();
  floor.blocks.forEach((b) => drawBlock(b, board));
  projectedPiece.blocks.forEach((b) => drawBlock(b, board, true));
  piece.blocks.forEach((b) => drawBlock(b, board));
  return board;
}

function drawBlock(block: Block, board: BoardType, projection = false) {
  if (block.y < 0) return;
  board[block.y][block.x] = projection ? "P" : block.type;
}

function getEmptyBoard(): BoardType {
  return Array(BOARD_HEIGHT)
    .fill([])
    .map(() => Array<BlockType>(BOARD_WIDTH).fill(""));
}
