export interface Event {
  id: string;
  event_type: string;
  entity_type?: string;
  entity_id?: string;
  metadata?: string;
  created_at: string;
}
