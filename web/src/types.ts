/**
 * Type definitions for BLD Solver structured data
 * 
 * These types match the Rust structures serialized from WASM
 * Using tagged enum format for better type discrimination
 */

// Corner Operations
export interface CornerSwapOperation {
    operation_type: 'Swap';
    data: {
        target1: number;
        target2: number;
        orientation: number;
    };
}

export interface CornerTwistOperation {
    operation_type: 'Twist';
    data: {
        target: number;
        orientation: number;
    };
}

export type CornerOperation = CornerSwapOperation | CornerTwistOperation;

// Edge Operations
export interface EdgeSwapOperation {
    operation_type: 'Swap';
    data: {
        target1: number;
        target2: number;
        orientation: number;
    };
}

export interface EdgeFlipOperation {
    operation_type: 'Flip';
    data: {
        target: number;
    };
}

export type EdgeOperation = EdgeSwapOperation | EdgeFlipOperation;

// Type guards
export function isCornerSwap(op: CornerOperation): op is CornerSwapOperation {
    return op.operation_type === 'Swap';
}

export function isCornerTwist(op: CornerOperation): op is CornerTwistOperation {
    return op.operation_type === 'Twist';
}

export function isEdgeSwap(op: EdgeOperation): op is EdgeSwapOperation {
    return op.operation_type === 'Swap';
}

export function isEdgeFlip(op: EdgeOperation): op is EdgeFlipOperation {
    return op.operation_type === 'Flip';
}

// Move Sequence
export interface MoveSequence {
    description: string;
    sequence: string;
}

// Solution Data (V2 - Structured)
export interface BldSolutionDataV2 {
    corner_operations: CornerOperation[];
    edge_operations: EdgeOperation[];
    move_sequences: MoveSequence[];
}

export interface BldSolutionResultV2 {
    success: boolean;
    error?: string;
    solution?: BldSolutionDataV2;
}

// Helper constants for display
export const TARGET_STICKERS_CORNER: string[][] = [
    ["UBL", "BUL", "LUB"], // 0
    ["UBR", "RUB", "BUR"], // 1
    ["UFR", "FUR", "RUF"], // 2
    ["UFL", "LUF", "FUL"], // 3
    ["DBL", "LDB", "BDL"], // 4
    ["DBR", "BDR", "RDB"], // 5
    ["DFR", "RDF", "FDR"], // 6
    ["DFL", "FDL", "LDF"], // 7
];

export const TARGET_STICKERS_EDGE: string[][] = [
    ["BL", "LB"], // 0
    ["BR", "RB"], // 1
    ["FR", "RF"], // 2
    ["FL", "LF"], // 3
    ["UB", "BU"], // 4
    ["UR", "RU"], // 5
    ["UF", "FU"], // 6
    ["UL", "LU"], // 7
    ["DB", "BD"], // 8
    ["DR", "RD"], // 9
    ["DF", "FD"], // 10
    ["DL", "LD"], // 11
];

// Helper functions for formatting
export function formatCornerOperation(op: CornerOperation): {
    type: string;
    details: string;
    meta: string;
    class: string;
} {
    if (isCornerSwap(op)) {
        const { target1, target2, orientation } = op.data;
        const sticker1 = TARGET_STICKERS_CORNER[target1][0];
        const sticker2 = TARGET_STICKERS_CORNER[target2][orientation];
        return {
            type: 'Corner Swap',
            details: `${sticker1} ↔ ${sticker2}`,
            meta: `Orientation: ${orientation}`,
            class: 'swap-card'
        };
    } else {
        const { target, orientation } = op.data;
        const sticker = TARGET_STICKERS_CORNER[target][0];
        const direction = orientation === 1 ? 'counter-clockwise' : 'clockwise';
        return {
            type: 'Corner Twist',
            details: sticker,
            meta: `Direction: ${direction}`,
            class: 'twist-card'
        };
    }
}

export function formatEdgeOperation(op: EdgeOperation): {
    type: string;
    details: string;
    meta: string;
    class: string;
} {
    if (isEdgeSwap(op)) {
        const { target1, target2, orientation } = op.data;
        const sticker1 = TARGET_STICKERS_EDGE[target1][0];
        const sticker2 = TARGET_STICKERS_EDGE[target2][orientation];
        return {
            type: 'Edge Swap',
            details: `${sticker1} ↔ ${sticker2}`,
            meta: `Orientation: ${orientation}`,
            class: 'swap-card'
        };
    } else {
        const { target } = op.data;
        const sticker = TARGET_STICKERS_EDGE[target][0];
        return {
            type: 'Edge Flip',
            details: sticker,
            meta: 'Flipped',
            class: 'flip-card'
        };
    }
}
