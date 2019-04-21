//TODO : Rely on List
fn ulTaskNotifyTake(xClearCountOnExit: &BaseType_t , xTicksToWait : &TickType_t) -> uint32_t {
    uint32_t ulReturn;

    taskENTER_CRITICAL();
    {
        /* Only block if the notification count is not already non-zero. */
        if( pxCurrentTCB.ulNotifiedValue_isZero())
        {
            /* Mark this task as waiting for a notification. */
            pxCurrentTCB.set_ucNotifyState (taskWAITING_NOTIFICATION);

            if( xTicksToWait > ( TickType_t ) 0 )
            {
                //TODO : Rely on List
                prvAddCurrentTaskToDelayedList( xTicksToWait, pdTRUE );
                traceTASK_NOTIFY_TAKE_BLOCK();

                /* All ports are written to allow a yield in a critical
                section (some will yield immediately, others wait until the
                critical section exits) - but it is not something that
                application code should ever do. */
                portYIELD_WITHIN_API();
            }
            else
            {
                mtCOVERAGE_TEST_MARKER();
            }
        }
        else
        {
            mtCOVERAGE_TEST_MARKER();
        }
    }
    taskEXIT_CRITICAL();

    taskENTER_CRITICAL();
    {
        traceTASK_NOTIFY_TAKE();
        ulReturn = pxCurrentTCB.get_ulNotifiedValue();

        if( ulReturn != 0UL )
        {
            if( xClearCountOnExit != pdFALSE )
            {
                pxCurrentTCB.set_ulNotifiedValue_zero ();
            }
            else
            {
                pxCurrentTCB.set_ulNotifiedValue (ulReturn - 1);
            }
        }
        else
        {
            mtCOVERAGE_TEST_MARKER();
        }

        pxCurrentTCB.set_ucNotifyState (taskNOT_WAITING_NOTIFICATION);
    }
    taskEXIT_CRITICAL();

    ulReturn
}

fn xTaskNotifyWait(aulBitsToClearOnEntryrg: uint32_t,:uint32_t) -> BaseType_t {
    BaseType_t xReturn
}
BaseType_t xTaskNotifyWait( uint32_t ulBitsToClearOnEntry, uint32_t ulBitsToClearOnExit, uint32_t *pulNotificationValue, TickType_t xTicksToWait )
{
BaseType_t xReturn;

    taskENTER_CRITICAL();
    {
        /* Only block if a notification is not already pending. */
        if( pxCurrentTCB->ucNotifyState != taskNOTIFICATION_RECEIVED )
        {
            /* Clear bits in the task's notification value as bits may get
            set	by the notifying task or interrupt.  This can be used to
            clear the value to zero. */
            pxCurrentTCB->ulNotifiedValue &= ~ulBitsToClearOnEntry;

            /* Mark this task as waiting for a notification. */
            pxCurrentTCB->ucNotifyState = taskWAITING_NOTIFICATION;

            if( xTicksToWait > ( TickType_t ) 0 )
            {
                prvAddCurrentTaskToDelayedList( xTicksToWait, pdTRUE );
                traceTASK_NOTIFY_WAIT_BLOCK();

                /* All ports are written to allow a yield in a critical
                section (some will yield immediately, others wait until the
                critical section exits) - but it is not something that
                application code should ever do. */
                portYIELD_WITHIN_API();
            }
            else
            {
                mtCOVERAGE_TEST_MARKER();
            }
        }
        else
        {
            mtCOVERAGE_TEST_MARKER();
        }
    }
    taskEXIT_CRITICAL();

    taskENTER_CRITICAL();
    {
        traceTASK_NOTIFY_WAIT();

        if( pulNotificationValue != NULL )
        {
            /* Output the current notification value, which may or may not
            have changed. */
            *pulNotificationValue = pxCurrentTCB->ulNotifiedValue;
        }

        /* If ucNotifyValue is set then either the task never entered the
        blocked state (because a notification was already pending) or the
        task unblocked because of a notification.  Otherwise the task
        unblocked because of a timeout. */
        if( pxCurrentTCB->ucNotifyState == taskWAITING_NOTIFICATION )
        {
            /* A notification was not received. */
            xReturn = pdFALSE;
        }
        else
        {
            /* A notification was already pending or a notification was
            received while the task was waiting. */
            pxCurrentTCB->ulNotifiedValue &= ~ulBitsToClearOnExit;
            xReturn = pdTRUE;
        }

        pxCurrentTCB->ucNotifyState = taskNOT_WAITING_NOTIFICATION;
    }
    taskEXIT_CRITICAL();

    return xReturn;
}

#endif /* configUSE_TASK_NOTIFICATIONS */
/*-----------------------------------------------------------*/

#if( configUSE_TASK_NOTIFICATIONS == 1 )

BaseType_t xTaskGenericNotify( TaskHandle_t xTaskToNotify, uint32_t ulValue, eNotifyAction eAction, uint32_t *pulPreviousNotificationValue )
{
TCB_t * pxTCB;
BaseType_t xReturn = pdPASS;
uint8_t ucOriginalNotifyState;

    configASSERT( xTaskToNotify );
    pxTCB = ( TCB_t * ) xTaskToNotify;

    taskENTER_CRITICAL();
    {
        if( pulPreviousNotificationValue != NULL )
        {
            *pulPreviousNotificationValue = pxTCB->ulNotifiedValue;
        }

        ucOriginalNotifyState = pxTCB->ucNotifyState;

        pxTCB->ucNotifyState = taskNOTIFICATION_RECEIVED;

        switch( eAction )
        {
            case eSetBits	:
                pxTCB->ulNotifiedValue |= ulValue;
                break;

            case eIncrement	:
                ( pxTCB->ulNotifiedValue )++;
                break;

            case eSetValueWithOverwrite	:
                pxTCB->ulNotifiedValue = ulValue;
                break;

            case eSetValueWithoutOverwrite :
                if( ucOriginalNotifyState != taskNOTIFICATION_RECEIVED )
                {
                    pxTCB->ulNotifiedValue = ulValue;
                }
                else
                {
                    /* The value could not be written to the task. */
                    xReturn = pdFAIL;
                }
                break;

            case eNoAction:
                /* The task is being notified without its notify value being
                updated. */
                break;
        }

        traceTASK_NOTIFY();

        /* If the task is in the blocked state specifically to wait for a
        notification then unblock it now. */
        if( ucOriginalNotifyState == taskWAITING_NOTIFICATION )
        {
            ( void ) uxListRemove( &( pxTCB->xStateListItem ) );
            prvAddTaskToReadyList( pxTCB );

            /* The task should not have been on an event list. */
            configASSERT( listLIST_ITEM_CONTAINER( &( pxTCB->xEventListItem ) ) == NULL );

            #if( configUSE_TICKLESS_IDLE != 0 )
            {
                /* If a task is blocked waiting for a notification then
                xNextTaskUnblockTime might be set to the blocked task's time
                out time.  If the task is unblocked for a reason other than
                a timeout xNextTaskUnblockTime is normally left unchanged,
                because it will automatically get reset to a new value when
                the tick count equals xNextTaskUnblockTime.  However if
                tickless idling is used it might be more important to enter
                sleep mode at the earliest possible time - so reset
                xNextTaskUnblockTime here to ensure it is updated at the
                earliest possible time. */
                prvResetNextTaskUnblockTime();
            }
            #endif

            if( pxTCB->uxPriority > pxCurrentTCB->uxPriority )
            {
                /* The notified task has a priority above the currently
                executing task so a yield is required. */
                taskYIELD_IF_USING_PREEMPTION();
            }
            else
            {
                mtCOVERAGE_TEST_MARKER();
            }
        }
        else
        {
            mtCOVERAGE_TEST_MARKER();
        }
    }
    taskEXIT_CRITICAL();

    return xReturn;
}

#endif /* configUSE_TASK_NOTIFICATIONS */
/*-----------------------------------------------------------*/

#if( configUSE_TASK_NOTIFICATIONS == 1 )

BaseType_t xTaskGenericNotifyFromISR( TaskHandle_t xTaskToNotify, uint32_t ulValue, eNotifyAction eAction, uint32_t *pulPreviousNotificationValue, BaseType_t *pxHigherPriorityTaskWoken )
{
TCB_t * pxTCB;
uint8_t ucOriginalNotifyState;
BaseType_t xReturn = pdPASS;
UBaseType_t uxSavedInterruptStatus;

    configASSERT( xTaskToNotify );

    /* RTOS ports that support interrupt nesting have the concept of a
    maximum	system call (or maximum API call) interrupt priority.
    Interrupts that are	above the maximum system call priority are keep
    permanently enabled, even when the RTOS kernel is in a critical section,
    but cannot make any calls to FreeRTOS API functions.  If configASSERT()
    is defined in FreeRTOSConfig.h then
    portASSERT_IF_INTERRUPT_PRIORITY_INVALID() will result in an assertion
    failure if a FreeRTOS API function is called from an interrupt that has
    been assigned a priority above the configured maximum system call
    priority.  Only FreeRTOS functions that end in FromISR can be called
    from interrupts	that have been assigned a priority at or (logically)
    below the maximum system call interrupt priority.  FreeRTOS maintains a
    separate interrupt safe API to ensure interrupt entry is as fast and as
    simple as possible.  More information (albeit Cortex-M specific) is
    provided on the following link:
    http://www.freertos.org/RTOS-Cortex-M3-M4.html */
    portASSERT_IF_INTERRUPT_PRIORITY_INVALID();

    pxTCB = ( TCB_t * ) xTaskToNotify;

    uxSavedInterruptStatus = portSET_INTERRUPT_MASK_FROM_ISR();
    {
        if( pulPreviousNotificationValue != NULL )
        {
            *pulPreviousNotificationValue = pxTCB->ulNotifiedValue;
        }

        ucOriginalNotifyState = pxTCB->ucNotifyState;
        pxTCB->ucNotifyState = taskNOTIFICATION_RECEIVED;

        switch( eAction )
        {
            case eSetBits	:
                pxTCB->ulNotifiedValue |= ulValue;
                break;

            case eIncrement	:
                ( pxTCB->ulNotifiedValue )++;
                break;

            case eSetValueWithOverwrite	:
                pxTCB->ulNotifiedValue = ulValue;
                break;

            case eSetValueWithoutOverwrite :
                if( ucOriginalNotifyState != taskNOTIFICATION_RECEIVED )
                {
                    pxTCB->ulNotifiedValue = ulValue;
                }
                else
                {
                    /* The value could not be written to the task. */
                    xReturn = pdFAIL;
                }
                break;

            case eNoAction :
                /* The task is being notified without its notify value being
                updated. */
                break;
        }

        traceTASK_NOTIFY_FROM_ISR();

        /* If the task is in the blocked state specifically to wait for a
        notification then unblock it now. */
        if( ucOriginalNotifyState == taskWAITING_NOTIFICATION )
        {
            /* The task should not have been on an event list. */
            configASSERT( listLIST_ITEM_CONTAINER( &( pxTCB->xEventListItem ) ) == NULL );

            if( uxSchedulerSuspended == ( UBaseType_t ) pdFALSE )
            {
                ( void ) uxListRemove( &( pxTCB->xStateListItem ) );
                prvAddTaskToReadyList( pxTCB );
            }
            else
            {
                /* The delayed and ready lists cannot be accessed, so hold
                this task pending until the scheduler is resumed. */
                vListInsertEnd( &( xPendingReadyList ), &( pxTCB->xEventListItem ) );
            }

            if( pxTCB->uxPriority > pxCurrentTCB->uxPriority )
            {
                /* The notified task has a priority above the currently
                executing task so a yield is required. */
                if( pxHigherPriorityTaskWoken != NULL )
                {
                    *pxHigherPriorityTaskWoken = pdTRUE;
                }
                else
                {
                    /* Mark that a yield is pending in case the user is not
                    using the "xHigherPriorityTaskWoken" parameter to an ISR
                    safe FreeRTOS function. */
                    xYieldPending = pdTRUE;
                }
            }
            else
            {
                mtCOVERAGE_TEST_MARKER();
            }
        }
    }
    portCLEAR_INTERRUPT_MASK_FROM_ISR( uxSavedInterruptStatus );

    return xReturn;
}

#endif /* configUSE_TASK_NOTIFICATIONS */
/*-----------------------------------------------------------*/

#if( configUSE_TASK_NOTIFICATIONS == 1 )

void vTaskNotifyGiveFromISR( TaskHandle_t xTaskToNotify, BaseType_t *pxHigherPriorityTaskWoken )
{
TCB_t * pxTCB;
uint8_t ucOriginalNotifyState;
UBaseType_t uxSavedInterruptStatus;

    configASSERT( xTaskToNotify );

    /* RTOS ports that support interrupt nesting have the concept of a
    maximum	system call (or maximum API call) interrupt priority.
    Interrupts that are	above the maximum system call priority are keep
    permanently enabled, even when the RTOS kernel is in a critical section,
    but cannot make any calls to FreeRTOS API functions.  If configASSERT()
    is defined in FreeRTOSConfig.h then
    portASSERT_IF_INTERRUPT_PRIORITY_INVALID() will result in an assertion
    failure if a FreeRTOS API function is called from an interrupt that has
    been assigned a priority above the configured maximum system call
    priority.  Only FreeRTOS functions that end in FromISR can be called
    from interrupts	that have been assigned a priority at or (logically)
    below the maximum system call interrupt priority.  FreeRTOS maintains a
    separate interrupt safe API to ensure interrupt entry is as fast and as
    simple as possible.  More information (albeit Cortex-M specific) is
    provided on the following link:
    http://www.freertos.org/RTOS-Cortex-M3-M4.html */
    portASSERT_IF_INTERRUPT_PRIORITY_INVALID();

    pxTCB = ( TCB_t * ) xTaskToNotify;

    uxSavedInterruptStatus = portSET_INTERRUPT_MASK_FROM_ISR();
    {
        ucOriginalNotifyState = pxTCB->ucNotifyState;
        pxTCB->ucNotifyState = taskNOTIFICATION_RECEIVED;

        /* 'Giving' is equivalent to incrementing a count in a counting
        semaphore. */
        ( pxTCB->ulNotifiedValue )++;

        traceTASK_NOTIFY_GIVE_FROM_ISR();

        /* If the task is in the blocked state specifically to wait for a
        notification then unblock it now. */
        if( ucOriginalNotifyState == taskWAITING_NOTIFICATION )
        {
            /* The task should not have been on an event list. */
            configASSERT( listLIST_ITEM_CONTAINER( &( pxTCB->xEventListItem ) ) == NULL );

            if( uxSchedulerSuspended == ( UBaseType_t ) pdFALSE )
            {
                ( void ) uxListRemove( &( pxTCB->xStateListItem ) );
                prvAddTaskToReadyList( pxTCB );
            }
            else
            {
                /* The delayed and ready lists cannot be accessed, so hold
                this task pending until the scheduler is resumed. */
                vListInsertEnd( &( xPendingReadyList ), &( pxTCB->xEventListItem ) );
            }

            if( pxTCB->uxPriority > pxCurrentTCB->uxPriority )
            {
                /* The notified task has a priority above the currently
                executing task so a yield is required. */
                if( pxHigherPriorityTaskWoken != NULL )
                {
                    *pxHigherPriorityTaskWoken = pdTRUE;
                }
                else
                {
                    /* Mark that a yield is pending in case the user is not
                    using the "xHigherPriorityTaskWoken" parameter in an ISR
                    safe FreeRTOS function. */
                    xYieldPending = pdTRUE;
                }
            }
            else
            {
                mtCOVERAGE_TEST_MARKER();
            }
        }
    }
    portCLEAR_INTERRUPT_MASK_FROM_ISR( uxSavedInterruptStatus );
}

#endif /* configUSE_TASK_NOTIFICATIONS */

/*-----------------------------------------------------------*/

#if( configUSE_TASK_NOTIFICATIONS == 1 )

BaseType_t xTaskNotifyStateClear( TaskHandle_t xTask )
{
TCB_t *pxTCB;
BaseType_t xReturn;

    /* If null is passed in here then it is the calling task that is having
    its notification state cleared. */
    pxTCB = prvGetTCBFromHandle( xTask );

    taskENTER_CRITICAL();
    {
        if( pxTCB->ucNotifyState == taskNOTIFICATION_RECEIVED )
        {
            pxTCB->ucNotifyState = taskNOT_WAITING_NOTIFICATION;
            xReturn = pdPASS;
        }
        else
        {
            xReturn = pdFAIL;
        }
    }
    taskEXIT_CRITICAL();

    return xReturn;
}

#endif

