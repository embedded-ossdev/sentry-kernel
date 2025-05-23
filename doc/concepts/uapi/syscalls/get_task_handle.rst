sys_get_task_handle
"""""""""""""""""""
.. _uapi_task_handle:

**API definition**

   .. code-block:: c
      :caption: C UAPI for get_task_handle syscall

      enum Status __sys_get_task_handle(uint32_t label);

**Usage**

   In Sentry, as explained in :ref:`Task terminology chapter <task_terminology>`, a task
   is unikely identified by its label, but can spawn sucessive jobs. Each of these jobs
   is being a dedicated instance of the same task, but at different moments of the
   system lifecycle.

   In order to communicate with another task without any confusion and in order to be
   sure that the starting point of the communication, end to the finishing point of the
   communication stays with the very same remote job instance, communication API is
   using a per-job unique identifier, based on the task label, but with complementary fields.

   As a consequence, before communicating with a remote task, knowing the
   remote task label, must ask the kernel for the currently remote job instance
   identifier of that task. This identifier is a task handle, and will be used for
   all communication.

   If the remote job terminates (whatever the reason is), the task handle will
   automatically be invalid for next communication requests, even if a new job has been
   respawned for the very same task. This is an easy way to detect remote failure or
   termination.

   This syscall returns the currently uptodate valid handle associated with the task
   uniquely identified by `label` on the system, and can be called multiple time if needed.

   .. code-block:: C
      :linenos:
      :caption: sample bare usage of sys_get_handle

      uint32_t my_peer_label=0xbabe;
      taskh_t my_peer_handle;
      if (__sys_get_handle(my_peer_label) != STATUS_OK) {
         // [...]
      }
      copy_from_kernel(&my_peer_handle, sizeof(taskh_t));
      __sys_send_signal(my_peer_handle, SIGNAL_POLL);

**Required capability**

   None.

**Return values**

   * STATUS_INVALID if the target task do not exist in the current task domain
   * STATUS_OK
