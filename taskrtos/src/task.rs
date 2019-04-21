fn prvAddCurrentTaskToDelayedList(xTicksToWait: &TickType_t, xCanBlockIndefinitely: &BaseType_t) {
    let mut xTimeToWake : TickType_t;
    let xConstTickCount : TickType_t = xTickCount;

    if (INCLUDE_xTaskAbortDelay){
        /* About to enter a delayed list, so ensure the ucDelayAborted flag is
		reset to pdFALSE so it can be detected as having been set to pdTRUE
		when the task leaves the Blocked state. */
        pxCurrentTCB.give_ucDelayAborted_false ();
        //TODO pxCurrentTCB tidy;
    }

    /* Remove the task from the ready list before adding it to the blocked list
	as the same list item is used for both lists. */
    //TODO : Rely on List
    if( uxListRemove( &( pxCurrentTCB->xStateListItem ) ) == ( UBaseType_t ) 0 )
	{
		/* The current task must be in a ready list, so there is no need to
		check, and the port reset macro can be called directly. */
		( pxCurrentTCB , uxTopReadyPriority ) = portRESET_READY_PRIORITY( pxCurrentTCB , uxTopReadyPriority );
	}
	else
	{
		mtCOVERAGE_TEST_MARKER();
	}

    if ( INCLUDE_vTaskSuspend == 1 )
	{
		if( ( xTicksToWait == portMAX_DELAY ) && ( xCanBlockIndefinitely != pdFALSE ) )
		{
			/* Add the task to the suspended task list instead of a delayed task
			list to ensure it is not woken by a timing event.  It will block
			indefinitely. */
            //TODO : Rely on List
			vListInsertEnd( &xSuspendedTaskList, &( pxCurrentTCB->xStateListItem ) );
		}
		else
		{
			/* Calculate the time at which the task should be woken if the event
			does not occur.  This may overflow but this doesn't matter, the
			kernel will manage it correctly. */
			xTimeToWake = xConstTickCount + xTicksToWait;

            //TODO : Rely on List
			/* The list item will be inserted in wake time order. */
			listSET_LIST_ITEM_VALUE( &( pxCurrentTCB->xStateListItem ), xTimeToWake );

			if( xTimeToWake < xConstTickCount )
			{
				/* Wake time has overflowed.  Place this item in the overflow
				list. */
                //TODO : Rely on List
				vListInsert( pxOverflowDelayedTaskList, &( pxCurrentTCB->xStateListItem ) );
			}
			else
			{
				/* The wake time has not overflowed, so the current block list
				is used. */
                //TODO : Rely on List
				vListInsert( pxDelayedTaskList, &( pxCurrentTCB->xStateListItem ) );

				/* If the task entering the blocked state was placed at the
				head of the list of blocked tasks then xNextTaskUnblockTime
				needs to be updated too. */
				if( xTimeToWake < xNextTaskUnblockTime )
				{
					xNextTaskUnblockTime = xTimeToWake;
				}
				else
				{
					mtCOVERAGE_TEST_MARKER();
				}
			}
		}
	}
	else /* INCLUDE_vTaskSuspend */
	{
		/* Calculate the time at which the task should be woken if the event
		does not occur.  This may overflow but this doesn't matter, the kernel
		will manage it correctly. */
		xTimeToWake = xConstTickCount + xTicksToWait;

		/* The list item will be inserted in wake time order. */
        //TODO : Rely on List
		listSET_LIST_ITEM_VALUE( &( pxCurrentTCB->xStateListItem ), xTimeToWake );

		if( xTimeToWake < xConstTickCount )
		{
			/* Wake time has overflowed.  Place this item in the overflow list. */
            //TODO : Rely on List
			vListInsert( pxOverflowDelayedTaskList, &( pxCurrentTCB->xStateListItem ) );
		}
		else
		{
			/* The wake time has not overflowed, so the current block list is used. */
            //TODO : Rely on List
			vListInsert( pxDelayedTaskList, &( pxCurrentTCB->xStateListItem ) );

			/* If the task entering the blocked state was placed at the head of the
			list of blocked tasks then xNextTaskUnblockTime needs to be updated
			too. */
			if( xTimeToWake < xNextTaskUnblockTime )
			{
				xNextTaskUnblockTime = xTimeToWake;
			}
			else
			{
				mtCOVERAGE_TEST_MARKER();
			}
		}

		/* Avoid compiler warning when INCLUDE_vTaskSuspend is not 1. */
		( void ) xCanBlockIndefinitely;
	}
	#endif /* INCLUDE_vTaskSuspend */
}

}
