type MovementHint = {
  label: string;
  color: string;
};

type StateHint = {
  label: string;
  color: string;
};

function formatBucketType(bucketType: string): string {
  return bucketType
    .split('_')
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(' ');
}

const MOVEMENT_MAP: Record<string, MovementHint> = {
  in: { label: 'In', color: 'var(--color-success)' },
  out: { label: 'Out', color: 'var(--color-error)' },
  held: { label: 'Held', color: 'var(--color-warning)' },
  across: { label: 'Across', color: 'var(--color-info)' }
};

function formatMovementType(movementType: string): MovementHint {
  const key = movementType.toLowerCase();
  return MOVEMENT_MAP[key] ?? { label: formatBucketType(movementType), color: 'var(--color-text-muted)' };
}

const STATE_MAP: Record<string, StateHint> = {
  completed: { label: 'Completed', color: 'var(--color-success)' },
  pending: { label: 'Pending', color: 'var(--color-warning)' },
  failed: { label: 'Failed', color: 'var(--color-error)' },
  cancelled: { label: 'Cancelled', color: 'var(--color-text-muted)' },
  reversed: { label: 'Reversed', color: 'var(--color-info)' },
  processing: { label: 'Processing', color: 'var(--color-info)' },
  approved: { label: 'Approved', color: 'var(--color-success)' },
  rejected: { label: 'Rejected', color: 'var(--color-error)' }
};

function formatState(state: string): StateHint {
  const key = state.toLowerCase();
  return STATE_MAP[key] ?? { label: formatBucketType(state), color: 'var(--color-text-muted)' };
}

export { formatBucketType, formatMovementType, formatState };
export type { MovementHint, StateHint };
