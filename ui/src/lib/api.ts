export const API_BASE = 'http://127.0.0.1:3000/api/v1';

export interface TargetGroup {
    id: string;
    name: string;
    description: string | null;
    created_at: string;
    updated_at: string;
}

export interface Target {
    id: string;
    group_id: string;
    address: string;
    created_at: string;
}

export interface Label {
    id: string;
    group_id: string;
    key: string;
    value: string;
    created_at: string;
}

export async function fetchGroups(fetchFn = fetch): Promise<TargetGroup[]> {
    const res = await fetchFn(`${API_BASE}/groups`);
    if (!res.ok) throw new Error('Failed to fetch groups');
    return res.json();
}

export async function fetchGroup(id: string, fetchFn = fetch): Promise<TargetGroup> {
    const res = await fetchFn(`${API_BASE}/groups/${id}`);
    if (!res.ok) throw new Error('Failed to fetch group');
    return res.json();
}

export async function fetchTargets(groupId: string, fetchFn = fetch): Promise<Target[]> {
    const res = await fetchFn(`${API_BASE}/groups/${groupId}/targets`);
    if (!res.ok) throw new Error('Failed to fetch targets');
    return res.json();
}

export async function fetchLabels(groupId: string, fetchFn = fetch): Promise<Label[]> {
    const res = await fetchFn(`${API_BASE}/groups/${groupId}/labels`);
    if (!res.ok) throw new Error('Failed to fetch labels');
    return res.json();
}

export async function createGroup(name: string, description: string | null = null, fetchFn = fetch): Promise<{id: string}> {
    const res = await fetchFn(`${API_BASE}/groups`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, description })
    });
    if (!res.ok) throw new Error('Failed to create group');
    return res.json();
}

export async function updateGroup(id: string, name: string, description: string | null = null, fetchFn = fetch): Promise<void> {
    const res = await fetchFn(`${API_BASE}/groups/${id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, description })
    });
    if (!res.ok) throw new Error('Failed to update group');
}

export async function deleteGroup(id: string, fetchFn = fetch): Promise<void> {
    const res = await fetchFn(`${API_BASE}/groups/${id}`, {
        method: 'DELETE'
    });
    if (!res.ok) throw new Error('Failed to delete group');
}

export async function addTarget(groupId: string, address: string, fetchFn = fetch): Promise<{id: string}> {
    const res = await fetchFn(`${API_BASE}/groups/${groupId}/targets`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ address })
    });
    if (!res.ok) throw new Error('Failed to add target');
    return res.json();
}

export async function removeTarget(groupId: string, address: string, fetchFn = fetch): Promise<void> {
    const res = await fetchFn(`${API_BASE}/groups/${groupId}/targets/${encodeURIComponent(address)}`, {
        method: 'DELETE'
    });
    if (!res.ok) throw new Error('Failed to remove target');
}

export async function upsertLabel(groupId: string, key: string, value: string, fetchFn = fetch): Promise<void> {
    const res = await fetchFn(`${API_BASE}/groups/${groupId}/labels`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ key, value })
    });
    if (!res.ok) throw new Error('Failed to upsert label');
}

export async function removeLabel(groupId: string, key: string, fetchFn = fetch): Promise<void> {
    const res = await fetchFn(`${API_BASE}/groups/${groupId}/labels/${encodeURIComponent(key)}`, {
        method: 'DELETE'
    });
    if (!res.ok) throw new Error('Failed to remove label');
}
