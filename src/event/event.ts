import { Subject } from "rxjs";

export class EventService {
  private static _instance: EventService;
  public customEvent = new Subject();
  constructor() {
    if (EventService._instance) {
      return EventService._instance
    }
    EventService._instance = this;
  }

  public dispatchEvent(payload: any): void {
    this.customEvent.next(payload)
  }


}